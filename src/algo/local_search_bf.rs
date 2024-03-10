use super::*;

pub struct LocalSearch<'a> {
    cost_mat: &'a Costs,
    palets: Palets,
    #[allow(unused)]
    trucks: Trucks,
}

impl<'a> LocalSearch<'a> {
    pub fn new(cost_mat: &'a Costs, palets: Palets) -> Self {
        LocalSearch {
            cost_mat,
            palets,
            trucks: Default::default(),
        }
    }

    pub fn run(&self) -> usize {
        let mut actual_sol = self.gen_sol();
        let mut best_sol = actual_sol.clone();
        let mut best_cost = cost(&self.cost_mat, &best_sol);

        let mut it = 0;

        let mut visitados = HashSet::new();
        let mut switch = true;

        'main_loop: while it < N_EVAL {
            let mut tries = MAX_TRIES;
            visitados.clear();
            let next_sol = loop {
                let next_sol = gen_neighbour(&actual_sol, switch);

                if cost(&self.cost_mat, &next_sol) < best_cost {
                    break next_sol;
                }

                if !visitados.insert(next_sol) {
                    tries -= 1;
                }

                if CBT {
                    switch = !switch;
                }

                if tries == 0 || it >= N_EVAL {
                    break 'main_loop;
                }
                it += 1;
            };

            best_sol = actual_sol;
            best_cost = cost(&self.cost_mat, &best_sol);
            actual_sol = next_sol;
        }

        for (i, t) in best_sol.iter().enumerate() {
            println!("  Truck {}: {:?}", i, t);
        }
        println!("Coste: {}", best_cost);
        best_cost
    }

    fn gen_sol(&self) -> Trucks {
        let mut new_sol = Trucks::default();
        for pal in self.palets.iter().cloned() {
            let mut to_truck = rng::next_usize() % N_TRUCKS;
            while new_sol[to_truck].len() >= TRUCK_CAP {
                to_truck = rng::next_usize() % N_TRUCKS;
            }
            new_sol[to_truck].push(pal);
        }
        new_sol
    }
}
