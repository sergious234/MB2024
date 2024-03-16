use super::*;

pub struct Greedy<'a> {
    cost_mat: &'a Costs,
    palets: Palets,
    #[allow(unused)]
    trucks: Trucks,
}

impl<'a> Greedy<'a> {
    pub fn new(cost_mat: &'a Costs, palets: Palets) -> Self {
        Self {
            cost_mat,
            palets,
            trucks: Default::default(),
        }
    }

    pub fn run(&self) -> Trucks {
        let mut sol = Trucks::default();
        let pals = self.palets.clone();

        let mut lens = [0; N_TRUCKS];

        for pal in pals.into_iter() {
            let mut best_truck = 0;
            let mut best_cost = usize::MAX;
            for (i, truck) in sol.iter().enumerate() {
                let len = lens[i];
                if len >= TRUCK_CAP {
                    continue;
                }

                let cost = if len == 0 {
                    self.cost_mat[0][pal as usize - 1]
                } else {
                    self.cost_mat[truck[len-1] as usize -1][pal as usize - 1]
                };

                if cost < best_cost {
                    best_cost = cost;
                    best_truck = i;
                }
            }


            let last = lens[best_truck]; //sol[best_truck].len();
            sol[best_truck][last] = pal;
            lens[best_truck] += 1;
        }

        let _c = cost(self.cost_mat, &sol);

        sol
    }
}
