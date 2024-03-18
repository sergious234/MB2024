use std::ops::Div;

use crate::algo::*;
use crate::rng::next_f64_range;

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

        let mut switch = true;
        let lt = 100;

        let init_temp = self.std_dev() / (N as f64).ln();
        let mut temp = init_temp;

        let ann_mech = AnnealingMech::ExponentialDescend;

        let aceptacion = |delta: f64, t: f64| ((-delta) / t).exp();
        let mut left_ann = 0;
        let mut left_its = 0;

        const TOTAL_ANN: usize = 50 * N;
        const TOTAL_ITS: usize = 5_000 * N;

        while left_ann < TOTAL_ANN && left_its < TOTAL_ITS {
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

        // println!("Evals: {}", left_its);
        // println!("Coste: {}", best_cost);
        best_cost
    }
}

impl SimulatedAnnealing<'_> {
    fn std_dev(&self) -> f64 {
        const ITEMS: f64 = ((N * (N - 1)) as f64) / 2.0;
        let sum: f64 = self
            .cost_mat
            .iter()
            .enumerate()
            .map(|(i, row)| row.iter().skip(i + 1).sum::<usize>())
            .sum::<usize>() as f64;

        let mean = sum / ITEMS;

        self.cost_mat
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .skip(i + 1)
                    .map(|&e| (e as f64 - mean).powi(2))
                    .sum::<f64>()
            })
            .sum::<f64>()
            .div(ITEMS)
            .sqrt()
            .floor()
    }
}
