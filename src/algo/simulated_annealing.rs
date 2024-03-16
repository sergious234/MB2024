use crate::rng::next_f64_range;
use crate::algo::*;


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
        let mut best_sol = gen_sol(&self.palets);
        let mut best_cost = cost(self.cost_mat, &best_sol);

        // let mut visitados = HashSet::new();
        let mut switch = true;
        let lt = 100;

        let init_temp = self.std_dev() / (N as f64).ln();
        let mut temp = init_temp;

        let ann_mech = AnnealingMech::ExponentialDescend;

        let aceptacion = |delta: f64, t: f64| ((-delta) / t).exp();
        let mut left_ann = 0;
        let mut left_its = 0;

        while left_ann < 50 * N && left_its < 5_000 * N {
            left_ann += 1;

            for _ in 0..lt {
                let sol_cand = gen_neighbour(&best_sol, switch);
                let cost_cand = cost(self.cost_mat, &sol_cand);

                left_its += 1;
                let delta_cost = cost_cand as f64 - best_cost as f64;

                if delta_cost < 0.0 || next_f64_range(0.0, 1.0) < aceptacion(delta_cost, temp) {
                    best_cost = cost_cand;
                    best_sol = sol_cand;
                }

                if CBT {
                    switch = !switch;
                }
            }

            temp = match ann_mech {
                AnnealingMech::ExponentialDescend => ann_mech.update(temp, ANN_CONST),
                _ => ann_mech.update(init_temp, left_its as f64),
            };
        }
        // println!("Coste: {}", best_cost);
        best_cost
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

        let mean = sum / (((N * (N - 1)) as f64) / 2.0);

        let mut variance: f64 = 0.0;
        for i in 0..self.cost_mat.len() {
            for j in i + 1..self.cost_mat[i].len() {
                variance += (self.cost_mat[i][j] as f64 - mean).powi(2);
            }
        }
        (variance / (N * (N - 1)) as f64 / 2.0).sqrt().floor()
    }
}
