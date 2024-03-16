use crate::rng::{next_f64_range, next_usize};

use super::*;

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
        let tabu_size = 100;

        let mut tabu_time = 4;

        let mut it = 0;

        let mut best_neigh_cost = elite_cost;
        let mut best_neigh_sol = elite;

        while it < N_EVAL {
            let mut candidates = vec![];

            for _ in 0..tabu_size {
                let from: usize = next_usize() % N_TRUCKS;
                let mut to: usize = next_usize() % N_TRUCKS;

                let truck_a: usize = next_usize() % N_TRUCKS;
                let truck_b: usize = next_usize() % N_TRUCKS;

                while to == from {
                    to = next_usize() % N_TRUCKS;
                }

                let cand_sol = gen_neighbour_2(&best_neigh_sol, true, truck_a, truck_b, from, to);
                let cand_cost = cost(self.cost_mat, &cand_sol);

                candidates.push(((from, to), cand_cost, cand_sol));
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
                    best_neigh_sol = gen_sol(&self.palets);
                } else if u < 0.5 {
                    best_neigh_sol = greedy_sol;
                } else {
                    best_neigh_sol = elite;
                }

                best_neigh_cost = cost(self.cost_mat, &best_neigh_sol);

                if u < 0.5 {
                    tabu_time += tabu_time / 2;
                } else {
                    tabu_time /= 2;
                    if tabu_time < 1 {
                        tabu_time = 1;
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
