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
            let sol = self.gen_sol2();
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

    fn gen_sol2(&mut self) -> Trucks {
        let mut new_sol = Trucks::default();
        let mut lens = [0; N_TRUCKS];

        let mut pal_indx = 0;
        loop {
            let to_truck: Vec<usize> = (0..20).map(|_| RNG::next() as usize % N_TRUCKS).collect();

            let mut i = 0;
            while i < 20 {
                let last = lens[to_truck[i]];
                if last < 14 {
                    new_sol[to_truck[i]][last] = self.palets[pal_indx];
                    lens[to_truck[i]] += 1;
                    pal_indx += 1;
                }
                i += 1;
            }

            if pal_indx >= self.palets.len() {
                break;
            }
        }
        new_sol
    }

    fn gen_sol(&mut self) -> Trucks {
        let mut new_sol = Trucks::default();
        let mut lens = [0; N_TRUCKS];

        for pal in self.palets.iter().cloned() {
            let mut to_truck = RNG::next() as usize % N_TRUCKS;
            let mut last = lens[to_truck];
            while last >= TRUCK_CAP {
                to_truck = RNG::next() as usize % N_TRUCKS;
                last = lens[to_truck];
            }
            new_sol[to_truck][last] = pal;
            lens[to_truck] += 1;
        }
        new_sol
    }
}
