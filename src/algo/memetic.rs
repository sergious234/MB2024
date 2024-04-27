use std::collections::HashMap;

use super::genetic::{Cromosoma, CrossType, GAGreedyP, K, N_ELITISM};
use super::*;
use crate::logger::DataLogger;
use rayon::prelude::*;

const N_CROMOSOMA: usize = 50; //genetic::N_CROMOSOMA;
const CROSS: CrossType = CrossType::OX;
const EVALS: usize = 50 * N;
const PAIRS: usize = N_CROMOSOMA / 2 - 2;
const ST: bool = false;
const N_GEN: usize = 100 * N;

const V: usize = 2;

/// Cantidad de la poblacion a la que se le aplica la busqueda local
const LS_N: usize = match V {
    1 => N_CROMOSOMA / 10 * 2,
    2 => N_CROMOSOMA,
    _ => unreachable!(),
};
const OPT_CD: usize = match V {
    1 => 1,
    2 => 10,
    _ => unreachable!(),
};

pub struct Memetic<'a> {
    poblacion: Vec<Cromosoma>,
    palets: &'a Palets,
    cost_mat: &'a Costs,
    name: &'a str,
}

impl<'a> Memetic<'a> {
    pub fn new(cost_mat: &'a Costs, palets: &'a Palets, name: &'a str) -> Self {
        let greedy = GAGreedyP::new(cost_mat, palets);
        let mut poblacion: Vec<Cromosoma> = (0..N_CROMOSOMA - 1)
            .map(|_| Cromosoma::get_random())
            .collect();

        poblacion.push(greedy.run());

        for p in poblacion.iter_mut() {
            p.cost(palets, cost_mat);
        }

        Self {
            poblacion,
            palets,
            cost_mat,
            name,
        }
    }

    pub fn run(&mut self) {
        let mut gen = 0;
        let bl = LocalSearchMM::new(self.cost_mat, self.palets);

        let mut cache: HashMap<Cromosoma, usize> = HashMap::new();
        let mut next_gen: Vec<Cromosoma> = vec![];

        let mut logger = DataLogger::new(self.name);

        while gen < N_GEN {
            gen += 1;

            if gen % 100 == 0 {
                println!("{}", cache.len());
            }

            self.poblacion.sort_by(|a, b| a.cost.cmp(&b.cost));
            if gen % 1 == 0 {
                let mean_cost = self.poblacion.iter().map(|e| e.cost).min().expect("E");
                logger.log(gen, mean_cost);
            }

            let mut rec_list: Vec<&Cromosoma> = vec![];

            // let mut next_gen: Vec<_> = self.poblacion[N_ELITISM..].iter().cloned().collect();
            let mut next_gen: Vec<Cromosoma> = if ST {
                self.get_next_gen_st()
            } else {
                self.gen_next_gen_mt()
            };

            for _ in 0..N_ELITISM {
                next_gen.push(self.poblacion.remove(0));
            }

            next_gen.sort_by(|a, b| a.cost.cmp(&b.cost));

            if ST && gen % OPT_CD == 0 {
                for i in 0..LS_N {
                    let mut optimized_sol = bl.run_with_start::<EVALS>(&next_gen[i]);
                    let cost = optimized_sol.cost(self.palets, self.cost_mat);
                    // cache.insert(optimized_sol.clone(), cost);
                    if cost < next_gen[i].cost {
                        next_gen[i] = optimized_sol;
                    }
                    /*
                    if cache.contains_key(&next_gen[i]) {
                        if cache.get(&next_gen[i]).unwrap() < &next_gen[i].cost {
                            let mut optimized_sol = bl.run_with_start::<EVALS>(&next_gen[i]);
                            optimized_sol.cost(self.palets, self.cost_mat);
                            next_gen[i] = optimized_sol;
                        }
                    } else {
                        let mut optimized_sol = bl.run_with_start::<EVALS>(&next_gen[i]);
                        let cost = optimized_sol.cost(self.palets, self.cost_mat);
                        cache.insert(optimized_sol.clone(), cost);
                        if cost < next_gen[i].cost {
                            next_gen[i] = optimized_sol;
                        }
                    }
                    */
                }
            } else if gen % OPT_CD == 0 {
                let mejoras = (0..LS_N)
                    .filter(|i| !cache.contains_key(&next_gen[*i]))
                    .collect::<Vec<_>>()
                    .into_par_iter()
                    .map(|i| {
                        let mut optimized_sol = bl.run_with_start::<EVALS>(&next_gen[i]);
                        optimized_sol.cost(self.palets, self.cost_mat);
                        (optimized_sol, i)
                        // cache.insert(optimized_sol.clone(), cost);
                    })
                    .filter(|(sol, i)| sol.cost < next_gen[*i].cost)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .fold(0, |acc, (sol, i)| {
                        cache.insert(next_gen[i].clone(), next_gen[i].cost);
                        next_gen[i] = sol;
                        acc + 1
                    });
                // println!("Mejoras {mejoras}");
            }

            //            if gen % 100 == 0 {
            //                println!("Pob: {}", self.poblacion.len());
            //                println!("Next gen: {}", next_gen.len());
            //                println!("{gen}");
            //            }

            self.poblacion.clear();

            self.poblacion.append(&mut next_gen);
            //self.poblacion.remove(0);
        }

        for p in self.poblacion.iter_mut() {
            p.cost(self.palets, self.cost_mat);
        }

        let best = self
            .poblacion
            .iter()
            .min_by(|a, b| a.cost.cmp(&b.cost))
            .unwrap();
        println!("GA: {}", best.cost);

        /*
        let mut sol = Trucks::default();
        for (truck, chunk) in sol.iter_mut().zip(best.palets.chunks(TRUCK_CAP)) {
            for i in 0..TRUCK_CAP {
                truck[i] = self.palets[chunk[i] as usize];
            }
        }

        println!(
            "BLGA: {}",
            cost(
                self.cost_mat,
                &LocalSearchBF::new(self.cost_mat, self.palets).run_with_start(sol)
            )
        );
        */
    }

    fn get_next_gen_st(&mut self) -> Vec<Cromosoma> {
        let mut next_gen = vec![];
        let mut rec_list = vec![];
        for _cross in 0..PAIRS {
            rec_list.clear();
            let mut index_list = vec![];
            let mut i = 0;
            while i < K {
                let index = RNG::next() % N_CROMOSOMA;
                let next = &self.poblacion[index];
                if !index_list.contains(&index) {
                    rec_list.push(next);
                    i += 1;
                }
            }

            let parent1 = *rec_list
                .iter()
                .min_by(|a, b| a.cost.cmp(&b.cost))
                .expect("Missing parent 1");

            i = 0;
            rec_list.clear();
            index_list = vec![];
            while i < K {
                let index = RNG::next() % N_CROMOSOMA;
                let next = &self.poblacion[RNG::next() % N_CROMOSOMA];
                if !index_list.contains(&index) {
                    rec_list.push(next);
                    i += 1;
                }
            }

            let parent2 = *rec_list
                .iter()
                .min_by(|a, b| a.cost.cmp(&b.cost))
                .expect("Missing parent 2");

            let cross_prob = RNG::next_f64();

            let mut offpring = if cross_prob <= 0.85 {
                CROSS.cross(&parent1, &parent2)
            } else {
                (parent1.clone(), parent2.clone())
            };

            offpring.0.cost(self.palets, self.cost_mat);
            offpring.1.cost(self.palets, self.cost_mat);

            next_gen.push(offpring.0);
            next_gen.push(offpring.1);
        }
        next_gen
    }

    fn gen_next_gen_mt(&mut self) -> Vec<Cromosoma> {
        (0..PAIRS)
            .into_par_iter()
            .map(|_| {
                let mut rec_list = vec![];
                let mut index_list = vec![];
                let mut i = 0;
                while i < K {
                    let index = RNG::next() % N_CROMOSOMA;
                    let next = &self.poblacion[index];
                    if !index_list.contains(&index) {
                        rec_list.push(next);
                        i += 1;
                    }
                }

                let parent1 = *rec_list
                    .iter()
                    .min_by(|a, b| a.cost.cmp(&b.cost))
                    .expect("Missing parent 1");

                i = 0;
                rec_list.clear();
                index_list = vec![];
                while i < K {
                    let index = RNG::next() % N_CROMOSOMA;
                    let next = &self.poblacion[RNG::next() % N_CROMOSOMA];
                    if !index_list.contains(&index) {
                        rec_list.push(next);
                        i += 1;
                    }
                }

                let parent2 = *rec_list
                    .iter()
                    .min_by(|a, b| a.cost.cmp(&b.cost))
                    .expect("Missing parent 2");

                let cross_prob = RNG::next_f64();

                let mut offpring: [Cromosoma; 2] = if cross_prob <= 0.85 {
                    CROSS
                        .cross(&parent1, &parent2)
                        .try_into()
                        .expect("Can not convert into [Cromosoma; 2]")
                } else {
                    [parent1.clone(), parent2.clone()]
                };

                offpring[0].cost(self.palets, self.cost_mat);
                offpring[1].cost(self.palets, self.cost_mat);

                offpring
            })
            .flatten_iter()
            .collect()
        // .flatten_iter()
        // .collect();
    }
}

// section -- LocalSearchBF Mem
pub struct LocalSearchMM<'a> {
    cost_mat: &'a Costs,
    palets: &'a Palets,
}

impl<'a> LocalSearchMM<'a> {
    pub fn new(cost_mat: &'a Costs, palets: &'a Palets) -> Self {
        Self { cost_mat, palets }
    }

    fn run_with_start<const EVALS: usize>(&self, sol: &Cromosoma) -> Cromosoma {
        let mut best_sol = sol.clone();
        let mut best_cost = best_sol.cost(self.palets, self.cost_mat);

        let mut it = 0;

        while it < EVALS {
            it += 1;
            let mut next_sol = Self::gen_neighbour(&best_sol, true);

            // if CBT {
            //     switch = !switch;
            // }

            let next_cost = next_sol.cost(self.palets, self.cost_mat);

            if next_cost < best_cost {
                best_sol = next_sol;
                best_cost = next_cost;
            }
        }
        best_sol
    }

    fn gen_neighbour(sol: &Cromosoma, change_palets: bool) -> Cromosoma {
        let mut new_sol = sol.clone();
        new_sol.cost = 0;

        if change_palets {
            let from_truck = RNG::next() % N_PALETS;
            let mut to_truck = RNG::next() % N_PALETS;
            while to_truck == from_truck {
                to_truck = RNG::next() % N_PALETS;
            }
            new_sol.palets.swap(from_truck, to_truck);
        } else {
            let truck = (RNG::next() % N_TRUCKS) * TRUCK_CAP;

            let from = (RNG::next() % TRUCK_CAP) + truck;
            let mut to = (RNG::next() % TRUCK_CAP) + truck;
            while to == from {
                to = RNG::next() % TRUCK_CAP;
            }

            new_sol.palets.swap(to, from);
        }
        new_sol
    }
}
// end section --
