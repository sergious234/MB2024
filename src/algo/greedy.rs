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

    pub fn run(&self) -> usize {
        let mut sol = Trucks::default();
        let mut pals = self.palets.clone();

        for truck in sol.iter_mut() {
            truck.push(pals.pop().expect("There are no palets left"));

            let mut current_city = truck[0];

            while truck.len() < TRUCK_CAP {
                let (index, _) = pals
                    .iter()
                    .enumerate()
                    .map(|(i, &p)| (i, self.cost_mat[current_city - 1][p - 1]))
                    .min_by(|(_i, dist_a), (_j, dist_b)| dist_a.partial_cmp(dist_b).unwrap())
                    .expect("You fucked up");

                let city = pals.remove(index);
                truck.push(city);
                current_city = city;
            }
        }

        for (i, t) in sol.iter().enumerate() {
            println!("  Truck {}: {:?}", i, t);
        }
        let c = cost(self.cost_mat, &sol);
        println!("Coste: {}", c);
        c
    }
}
