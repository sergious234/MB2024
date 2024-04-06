use super::*;

#[allow(dead_code)]
pub struct GreedyExp<'a> {
    cost_mat: &'a Costs,
    palets: Palets,
    #[allow(unused)]
    trucks: Trucks,
}

#[allow(dead_code)]
impl<'a> GreedyExp<'a> {
    pub fn new(cost_mat: &'a Costs, palets: Palets) -> Self {
        Self {
            cost_mat,
            palets,
            trucks: Default::default(),
        }
    }

    pub fn run(&self) -> Trucks {
        let mut sol = Trucks::default();
        let mut pals = read_palets(PALETS_PATH); //self.palets.clone();

        let mut lens = [0; N_TRUCKS];

        for (it, truck) in sol.iter_mut().enumerate() {
            truck[0] = pals.pop().expect("There are no palets left");
            lens[it] += 1;

            let mut current_city = truck[0];

            while lens[it] < TRUCK_CAP {
                let (index, _) = pals
                    .iter()
                    .enumerate()
                    .map(|(i, &p)| {
                        (
                            i,
                            self.cost_mat[(current_city - 1) as usize][(p - 1) as usize],
                        )
                    })
                    .min_by(|(_i, dist_a), (_j, dist_b)| dist_a.partial_cmp(dist_b).unwrap())
                    .expect("You fucked up");

                let city = pals.remove(index);
                truck[lens[it]] = city;
                current_city = city;
                lens[it] += 1;
            }
        }

        let c = cost(self.cost_mat, &sol);
        println!("Coste: {}", c);
        sol
    }
}
