use super::*;

pub struct LocalSearch<'a> {
    cost_mat: &'a Costs,
    palets: &'a Palets,
    #[allow(unused)]
    trucks: Trucks,
}

impl<'a> LocalSearch<'a> {
    pub fn new(cost_mat: &'a Costs, palets: &'a Palets) -> Self {
        LocalSearch {
            cost_mat,
            palets,
            trucks: Default::default(),
        }
    }

    pub fn run_with_start(&self, mut best_sol: Trucks) -> Trucks {
        let mut best_cost = cost(self.cost_mat, &best_sol);

        let mut it = 0;
        let mut best_it = 0;

        // let mut visitados = HashSet::new();
        let mut switch = true;

        while it < N_EVAL {
            it += 1;
            let next_sol = gen_neighbour(&best_sol, switch);

            if CBT {
                switch = !switch;
            }

            let next_cost = cost(self.cost_mat, &next_sol);

            if next_cost < best_cost {
                best_sol = next_sol;
                best_cost = next_cost;
                best_it = it;
            }
        }
        best_sol
    }

    pub fn run(&self) -> usize {
        let mut best_sol = gen_sol2(&self.palets);
        let mut best_cost = cost(self.cost_mat, &best_sol);

        let mut it = 0;
        let mut best_it = 0;

        // let mut visitados = HashSet::new();
        let mut switch = true;

        while it < N_EVAL {
            it += 1;
            let next_sol = gen_neighbour(&best_sol, switch);

            if CBT {
                switch = !switch;
            }

            let next_cost = cost(self.cost_mat, &next_sol);

            if next_cost < best_cost {
                best_sol = next_sol;
                best_cost = next_cost;
                best_it = it;
            }
        }

        println!("BI: {}", best_it);
        println!("Coste: {}", best_cost);
        best_cost
    }
}
