use crate::rng::{next_f64_range, next_usize};

use super::*;

type LongMemory = [[u16; N]; N_TRUCKS];

#[allow(dead_code)]
pub struct TabuSearch<'a> {
    cost_mat: &'a Costs,
    palets: Palets,
    #[allow(unused)]
    trucks: Trucks,
    tabu_mat: Vec<Vec<usize>>,
}

#[allow(dead_code)]
impl<'a> TabuSearch<'a> {
    pub fn new(cost_mat: &'a Costs, palets: Palets) -> Self {
        let mut mat = vec![];
        for i in 0..N {
            mat.push(vec![]);
            for _ in 0..N {
                mat[i].push(0);
            }
        }

        Self {
            cost_mat,
            palets,
            trucks: Default::default(),
            tabu_mat: mat,
        }
    }

    pub fn run(&mut self) -> usize {
        let greedy_sol = Greedy::new(self.cost_mat, self.palets.clone()).run();

        let mut elite = gen_sol(&self.palets);
        let mut elite_cost = cost(self.cost_mat, &elite);
        let tabu_size = 50;

        let mut tabu_time = 4;

        let mut it = 0;

        let mut best_neigh_cost = elite_cost;
        let mut best_neigh_sol = elite;

        let mut moves = [[0u16; N]; N_TRUCKS];

        while it < N_EVAL * 10 {
            let mut candidates = vec![];

            for _ in 0..tabu_size {
                let from: usize = next_usize() % TRUCK_CAP;
                let mut to: usize = next_usize() % TRUCK_CAP;

                let truck_a: usize = next_usize() % N_TRUCKS;
                let truck_b: usize = next_usize() % N_TRUCKS;

                while to == from {
                    to = next_usize() % N_TRUCKS;
                }

                let cand_sol = gen_neighbour_2(&best_neigh_sol, true, truck_a, truck_b, from, to);
                let cand_cost = cost(self.cost_mat, &cand_sol);

                candidates.push(((from, to, truck_a, truck_b), cand_cost, cand_sol));
                it += 1;
            }

            // Sort by cost
            candidates.sort_by(|a, b| a.1.cmp(&b.1));

            for cand in candidates {
                let is_tabu = self.tabu_mat[cand.0 .0][cand.0 .1] > 0;

                if !is_tabu {
                    best_neigh_sol = cand.2;
                    best_neigh_cost = cand.1;
                    self.tabu_mat[cand.0 .0][cand.0 .1] = tabu_time;

                    let truck_a = cand.0 .2;
                    let truck_b = cand.0 .3;
                    let pal_a = best_neigh_sol[truck_a][cand.0 .0] as usize - 1;
                    let pal_b = best_neigh_sol[truck_b][cand.0 .1] as usize - 1;

                    moves[truck_a][pal_b] += 1;
                    moves[truck_b][pal_a] += 1;
                    break;
                }

                // NOTE: Fails about 20 times because of the Tabu Mat in
                // the Large file.
            }

            for row in self.tabu_mat.iter_mut() {
                for element in row.iter_mut() {
                    if *element > 0 {
                        *element -= 1;
                    }
                }
            }

            if it % (500 * N) == 0 {
                let u = next_f64_range(0.0, 1.0);

                if u < 0.25 {
                    best_neigh_sol = self.long_memory_sol(&moves);
                } else if u < 0.5 {
                    best_neigh_sol = elite;
                } else {
                    best_neigh_sol = greedy_sol;
                }

                best_neigh_cost = cost(self.cost_mat, &best_neigh_sol);
                it += 1;

                if u < 0.5 {
                    tabu_time += tabu_time / 2;
                } else {
                    tabu_time /= 2;
                    if tabu_time < 1 {
                        tabu_time = 1;
                    }
                }

                for row in self.tabu_mat.iter_mut() {
                    for item in row {
                        *item = 0;
                    }
                }
            }

            if best_neigh_cost < elite_cost {
                elite = best_neigh_sol;
                elite_cost = best_neigh_cost;
            }
        }

        println!("Coste: {}", elite_cost);
        elite_cost
    }
}

impl TabuSearch<'_> {
    pub fn long_memory_sol(&self, memory: &LongMemory) -> Trucks {
        let mut pals = [0; N];
        let mut new_sol = Trucks::default();

        for city in &self.palets {
            pals[*city as usize - 1] += 1;
        }

        for (i, t) in memory.iter().enumerate() {
            let mut sorted: Vec<(usize, &u16)> = t.iter().enumerate().collect();
            sorted.sort_by(|a, b| a.1.cmp(b.1));

            let mut last = 0;
            while last < TRUCK_CAP {
                for (pos, _pal) in sorted.iter().cloned() {
                    if last >= TRUCK_CAP {
                        break;
                    }
                    if pals[pos] > 0 {
                        new_sol[i][last] = (pos as u8) + 1;
                        last += 1;
                        pals[pos] -= 1;
                    }
                }
            }
        }

        /*
        println!("LongMemory");
        for truck in new_sol {
            println!("{:?}", truck);
        }
        */

        new_sol
    }
}
