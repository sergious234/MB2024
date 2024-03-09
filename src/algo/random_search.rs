use super::*;

pub struct RandomSearch<'a> {
    cost_mat: &'a Costs,
    palets: Palets,
    #[allow(unused)]
    trucks: Trucks,
}

impl<'a> RandomSearch<'a> {
    pub fn new(cost_mat: &'a Costs, palets: Palets) -> Self {
        RandomSearch {
            cost_mat,
            palets,
            trucks: Default::default(),
        }
    }

    pub fn run(&self) {
        let mut sol = self.gen_sol();
        let mut it = 0;

        let mut best_sol = sol;
        let mut best_cost = self.cost(&best_sol);
        while it < N_EVAL {
            sol = self.gen_sol();
            let new_cost = self.cost(&sol);

            if new_cost < best_cost {
                best_sol = sol;
                best_cost = new_cost;
            }
            it += 1;
        }

        for (i, t) in best_sol.iter().enumerate() {
            println!("Truck {}: {:?}", i, t);
        }
        println!("Coste: {}", best_cost)
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

    fn cost(&self, sol: &Trucks) -> usize {
        let mut cost = 0;
        for truck in sol.iter() {
            let mut actual_city = 0;
            for city in truck.iter().map(|e| e - 1) {
                cost += self.cost_mat[actual_city][city];
                actual_city = city;
            }
            cost += self.cost_mat[actual_city][0];
        }
        cost
    }
}
