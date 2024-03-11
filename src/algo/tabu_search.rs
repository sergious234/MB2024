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
        let mut best_sol = self.gen_sol();
        let mut best_cost = cost(self.cost_mat, &best_sol);

        let mut it = 0;
        let is_aspirant = |cost, best_cost| cost < best_cost - 200;

        while it < N_EVAL {
            let mut candidates = vec![];

            for _ in 0..30 {
                let from: usize = rand::random::<usize>() % N_TRUCKS;
                let mut to: usize = rand::random::<usize>() % N_TRUCKS;

                let truck_a: usize = rand::random::<usize>() % N_TRUCKS;
                let truck_b: usize = rand::random::<usize>() % N_TRUCKS;

                while to == from {
                    to = rand::random::<usize>() % N_TRUCKS;
                }
                let cand_sol = gen_neighbour_2(&best_sol, true, truck_a, truck_b, from, to);
                let cand_cost = cost(self.cost_mat, &cand_sol);

                candidates.push(((from, to), cand_cost, cand_sol));
                it += 1;
            }

            // Sort by cost
            candidates.sort_by(|a, b| a.1.cmp(&b.1));

            let mut best_neigh_cost = usize::MAX;
            let mut best_neigh_sol = None;

            for cand in candidates {
                let is_tabu = self.tabu_mat[cand.0 .0][cand.0 .1] > 0;

                // TODO: Ask if a worst candidate should always be selected or only
                // if it meets a criteria
                if !is_tabu && (cand.1 < best_cost || cand.1 >= best_cost + best_cost / 4) {
                    best_neigh_sol = Some(cand.2);
                    best_neigh_cost = cand.1;
                    self.tabu_mat[cand.0 .0][cand.0 .1] = 3;
                    break;
                }
                if is_tabu && is_aspirant(cand.1, best_cost) {
                    best_neigh_sol = Some(cand.2);
                    best_neigh_cost = cand.1;
                    self.tabu_mat[cand.0 .0][cand.0 .1] = 3;
                    break;
                }

                // NOTE: Fails about 45k times because of the Tabu Mat in
                // the Large file.
            }

            for row in self.tabu_mat.iter_mut() {
                for element in row.iter_mut() {
                    if *element > 0 {
                        *element -= 1;
                    }
                }
            }

            if best_neigh_cost < best_cost {
                best_sol = best_neigh_sol.expect("No neighbour was found! :( ");
                best_cost = best_neigh_cost;
            }
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
