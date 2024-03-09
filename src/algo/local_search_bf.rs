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

    pub fn run(&self) {
        let mut actual_sol = self.gen_sol();
        let mut best_sol = actual_sol.clone();
        let mut best_cost = self.cost(&best_sol);

        let mut it = 0;

        let mut visitados = HashSet::new();
        let mut switch = true;
        'main_loop: while it < N_EVAL {
            let mut next_sol = gen_neighbour(&actual_sol, switch);
            visitados.clear();
            visitados.insert(next_sol.clone());

            while self.cost(&next_sol) >= best_cost {
                next_sol = gen_neighbour(&actual_sol, switch);
                let mut tries = MAX_TRIES;
                while visitados.contains(&next_sol) && tries > 0 {
                    next_sol = gen_neighbour(&actual_sol, switch);
                    tries -= 1;
                }
                if tries == 0 {
                    break 'main_loop;
                }
                visitados.insert(next_sol.clone());
            }

            if CBT {
                switch = !switch;
            }

            best_sol = actual_sol.clone();
            best_cost = self.cost(&best_sol);
            actual_sol = next_sol;
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


