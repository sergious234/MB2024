use super::*;

pub struct RandomSearch<'a> {
    cost_mat: &'a Costs,
    palets: Palets,
    #[allow(unused)]
    trucks: Trucks,
}

impl<'a> RandomSearch<'a> {
    pub fn new(cost_mat: &'a Costs, palets: Palets) -> Self {
        Self {
            cost_mat,
            palets,
            trucks: Default::default(),
        }
    }

    pub fn run(&mut self) -> usize {
        let mut it = 0;

        let mut _best_sol = self.gen_sol();
        let mut best_cost = cost(self.cost_mat, &_best_sol);
        let mut best_it = 0;
        while it < N_EVAL {
            let sol = self.gen_sol();
            let new_cost = cost(self.cost_mat, &sol);

            if new_cost < best_cost {
                _best_sol = sol;
                best_cost = new_cost;
                best_it = it;
            }
            it += 1;
        }

        println!("BI {}", best_it);
        println!("Coste: {}", best_cost);
        best_cost
    }

    fn gen_sol(&mut self) -> Trucks {
        let mut new_sol = Trucks::default();
        let mut lens = [0; N_TRUCKS];
        let mut randoms = RNG::cicle();

        for pal in self.palets.iter().cloned() {
            while let Some(to_truck) = randoms.next() {
                let to_truck = to_truck % N_TRUCKS;
                let last = lens[to_truck];

                if last < TRUCK_CAP {
                    new_sol[to_truck][last] = pal;
                    lens[to_truck] += 1;
                    break;
                }
            }
        }
        new_sol
    }
}
