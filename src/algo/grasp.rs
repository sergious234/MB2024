use super::*;
pub struct Grasp<'a> {
    cost_mat: &'a Costs,
    palets: &'a Palets,
    #[allow(unused)]
    trucks: Trucks,
}

impl<'a> Grasp<'a> {
    pub fn new(cost_mat: &'a Costs, palets: &'a Palets) -> Self {
        Self {
            cost_mat,
            palets,
            trucks: Trucks::default(),
        }
    }

    pub fn run(&self) -> Trucks {
        let mut best_sol = Trucks::default();
        let mut best_cost = usize::MAX;

        let gp = GreedyT::new(self.cost_mat, self.palets);
        let ls = LocalSearchBF::new(self.cost_mat, self.palets);
        for _it in 0..50 {
            let mut sol = gp.run();
            sol = ls.run_with_start(sol);
            let cost = cost(self.cost_mat, &sol);

            if cost < best_cost {
                best_cost = cost;
                best_sol = sol;
            }
        }
        println!("Coste: {}", best_cost);
        best_sol
    }
}

/*
 *
 * =================================================
 *
 */

struct GreedyT<'a> {
    cost_mat: &'a Costs,
    palets: &'a Palets,
    #[allow(unused)]
    trucks: Trucks,
}

impl<'a> GreedyT<'a> {
    pub fn new(cost_mat: &'a Costs, palets: &'a Palets) -> Self {
        Self {
            cost_mat,
            palets,
            trucks: Trucks::default(),
        }
    }

    pub fn run(&self) -> Trucks {
        let mut sol = Trucks::default();

        let pals = self.palets.clone();

        let mut lens = [0; N_TRUCKS];

        for pal in pals.iter().copied() {
            let mut distances = vec![];
            for (i, t) in sol.iter().enumerate() {
                if lens[i] < TRUCK_CAP {
                    let t_len = lens[i];
                    if t_len == 0 {
                        distances.push((self.cost_mat[0][pal as usize - 1], i));
                    } else {
                        let last = t[t_len - 1];
                        distances.push((self.cost_mat[last as usize - 1][pal as usize - 1], i));
                    }
                }
            }
            distances.sort_by(|a, b| a.0.cmp(&b.0));
            let best_trucks: Vec<_> = distances.iter().map(|e| e.1).take(N_TRUCKS / 2).collect();
            let prob = RNG::next() % best_trucks.len();
            let truck = best_trucks[prob];

            sol[truck][lens[truck]] = pal;
            lens[truck] += 1;
        }

        let _c = cost(self.cost_mat, &sol);
        sol
    }
}

/*
 *
 * =================================================
 *
 */

struct GreedyP<'a> {
    cost_mat: &'a Costs,
    palets: &'a Palets,
    #[allow(unused)]
    trucks: Trucks,
}

impl<'a> GreedyP<'a> {
    pub fn new(cost_mat: &'a Costs, palets: &'a Palets) -> Self {
        Self {
            cost_mat,
            palets,
            trucks: Trucks::default(),
        }
    }

    pub fn run(&self) -> Trucks {
        let mut sol = Trucks::default();
        let mut pals = [0; N];
        for pal in self.palets.iter().cloned() {
            pals[pal as usize - 1] += 1;
        }
        let mut lens = [0; N_TRUCKS];

        let mut pal_costs = vec![];
        for (i, truck) in sol.iter_mut().enumerate() {
            while lens[i] < TRUCK_CAP {
                pal_costs.clear();

                for pal in self.palets.iter().cloned() {
                    if pals[pal as usize - 1] > 0 {
                        let t_len = lens[i];
                        let last = if t_len == 0 {
                            0
                        } else {
                            truck[t_len - 1] as usize - 1
                        };
                        pal_costs.push((self.cost_mat[last][pal as usize - 1], pal));
                    }
                }

                pal_costs.sort_by(|a, b| a.0.cmp(&b.0));
                let best_palets: Vec<u8> = pal_costs
                    .iter()
                    .map(|e| e.1)
                    .take((pals.iter().sum::<usize>() as f64 * 0.1).ceil() as usize)
                    .collect();
                let last = lens[i];
                let chosen_one = *best_palets.get(RNG::next() % best_palets.len()).unwrap();

                truck[last] = chosen_one;
                lens[i] += 1;
                pals[chosen_one as usize - 1] -= 1;
            }
        }
        sol
    }
}
