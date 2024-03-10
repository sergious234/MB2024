use super::*;

pub struct SimulatedAnnealing<'a> {
    cost_mat: &'a Costs,
    palets: Palets,
    #[allow(unused)]
    trucks: Trucks,
}

#[allow(unused)]
enum AnnealingMech {
    ExponentialDescend,
    BoltzmannCriteria,
    CaucheEscheme,
}

impl AnnealingMech {
    pub fn update(&self, temp: f64, alpha_iter: f64) -> f64 {
        match self {
            Self::ExponentialDescend => alpha_iter * temp,
            Self::BoltzmannCriteria => temp / (1.0 + alpha_iter.log10()),
            Self::CaucheEscheme => temp / (1.0 + alpha_iter),
        }
    }
}

impl<'a> SimulatedAnnealing<'a> {
    pub fn new(cost_mat: &'a Costs, palets: Palets) -> Self {
        Self {
            cost_mat,
            palets,
            trucks: Default::default(),
        }
    }

    pub fn run(&self) -> usize {
        let mut best_sol = self.gen_sol();
        let mut best_cost = cost(&self.cost_mat, &best_sol);

        // let mut visitados = HashSet::new();
        let mut switch = true;
        let lt = 20;

        let init_temp = self.std_dev() / (N as f64).ln();
        let mut temp = init_temp;

        let ann_mech = AnnealingMech::ExponentialDescend;

        let aceptacion = |delta: f64, t: f64| ((-delta) / t).exp();
        let mut left_its = (50 * N) as isize;

        let mut upgrade = true;
        while upgrade && left_its > 0 {
            upgrade = false;

            for cont in 0..lt {
                let sol_cand = gen_neighbour(&best_sol, switch);

                let cost_cand = cost(&self.cost_mat, &sol_cand);
                left_its -= 1;

                // Se calcula al reves debido a que buscamos un minimo no un maximo.
                let delta_cost = cost_cand as f64 - best_cost as f64;

                if delta_cost < 0.0 || rng::next_f64_range(0.0, 1.0) < aceptacion(delta_cost, temp)
                {
                    best_cost = cost_cand;
                    best_sol = sol_cand;
                    upgrade = true;
                }

                if CBT {
                    switch = !switch;
                }

                temp = match ann_mech {
                    AnnealingMech::ExponentialDescend => ann_mech.update(temp, ANN_CONST),
                    _ => ann_mech.update(init_temp, cont as f64),
                };
            }
        }

        /*
                for (i, t) in best_sol.iter().enumerate() {
                    println!("  Truck {}: {:?}", i, t);
                }
                println!("Coste: {}", best_cost);
        */
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

impl SimulatedAnnealing<'_> {
    fn std_dev(&self) -> f64 {
        let sum: f64 = self
            .cost_mat
            .iter()
            .enumerate()
            .map(|(i, row)| row.iter().skip(i + 1).sum::<usize>())
            .sum::<usize>() as f64;

        /*
        let mut sum: f64 = 0.0;
        for i in 0..self.cost_mat.len() {
            for j in i + 1..self.cost_mat[i].len() {
                sum += self.cost_mat[i][j] as f64;
            }
        }
        */

        let mean = sum / (((N * (N - 1)) as f64) / 2.0);

        let mut variance = 0.0;
        for i in 0..self.cost_mat.len() {
            for j in i..self.cost_mat[i].len() {
                variance += (self.cost_mat[i][j] as f64 - mean).powi(2);
            }
        }
        (variance / 2.0).sqrt().floor()
    }
}
