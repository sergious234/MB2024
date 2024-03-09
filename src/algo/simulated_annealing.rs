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
    pub fn update(&self, temp: usize, alpha_iter: f64) -> usize {
        match self {
            Self::ExponentialDescend => alpha_iter as usize * temp,
            Self::BoltzmannCriteria => (temp as f64 / (1.0 + alpha_iter.log10())).floor() as usize,
            Self::CaucheEscheme => (temp as f64 / (1.0 + alpha_iter)).floor() as usize,
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

    pub fn run(&self) {
        let mut best_sol = self.gen_sol();
        let mut best_cost = self.cost(&best_sol);

        let mut visitados = HashSet::new();
        let mut switch = true;
        let lt = 20;

        let init_temp = (self.std_dev() / (N as f64).ln() ) as usize;
        let mut temp = init_temp;

        let ann_mech = AnnealingMech::ExponentialDescend;

        let aceptacion = |delta: isize, t: usize| (-(delta as f64) / t as f64).exp() as usize;
        let mut left_its = 50 * N;

        let mut upgrade = true;
        while /*temp > 0 && */ upgrade && left_its > 0 {
            left_its -= 1;
            upgrade = false;

            for cont in 0..lt {
                let mut sol_cand = gen_neighbour(&best_sol, switch);
                let mut tries = MAX_TRIES;

                while visitados.contains(&sol_cand) && tries > 0 {
                    sol_cand = gen_neighbour(&best_sol, switch);
                    tries -= 1;
                }

                if tries == 0 {
                    break;
                }

                visitados.insert(sol_cand.clone());
                let cost_cand = self.cost(&sol_cand);

                // Se calcula al reves debido a que buscamos un minimo no un maximo.
                let delta_cost = cost_cand as isize - best_cost as isize;

                if delta_cost < 0 || rng::next_usize_range(0, 1) < aceptacion(delta_cost, temp) {
                    best_cost = cost_cand;
                    best_sol = sol_cand;
                    upgrade = true;

                    // Para no repetir candidatos
                    visitados.clear();
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

        for (i, t) in best_sol.iter().enumerate() {
            println!("Truck {}: {:?}", i, t);
        }
        println!("Coste: {}", best_cost);
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

impl SimulatedAnnealing<'_> {
    fn std_dev(&self) -> f64 {
        let mut sum: f64 = 0.0;
        for i in 0..self.cost_mat.len() {
            for j in i+1..self.cost_mat[i].len() {
                sum += self.cost_mat[i][j] as f64;
            }
        }

        let mean = sum / (((N*N+1) as f64) / 2.0);

        let mut variance = 0.0;
        for i in 0..self.cost_mat.len() {
            for j in i..self.cost_mat[i].len() {
                variance += (self.cost_mat[i][j] as f64 - mean).powi(2);
            }
        }
        (variance / 2.0).sqrt().floor()
    }
}
