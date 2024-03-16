use std::borrow::BorrowMut;

use rand::{
    rngs::{SmallRng, StdRng}, Rng, RngCore, SeedableRng
};

use super::*;

pub struct RandomSearch<'a> {
    cost_mat: &'a Costs,
    palets: Palets,
    #[allow(unused)]
    trucks: Trucks,
    rng: &'a mut SmallRng
}

impl<'a> RandomSearch<'a> {
    pub fn new(cost_mat: &'a Costs, palets: Palets, rng: &'a mut SmallRng) -> Self {
        Self {
            cost_mat,
            palets,
            trucks: Default::default(),
            rng
        }
    }

    pub fn run(&mut self) -> usize {
        let mut it = 0;

        let mut _best_sol = self.gen_sol();
        let mut best_cost = cost(self.cost_mat, &_best_sol);
        while it < N_EVAL {
            let sol = self.gen_sol();
            let new_cost = cost(self.cost_mat, &sol);

            if new_cost < best_cost {
                _best_sol = sol;
                best_cost = new_cost;
            }
            it += 1;
        }

        println!("Coste: {}", best_cost);
        best_cost
    }

    fn gen_sol(&mut self) -> Trucks {
        let mut new_sol = Trucks::default();
        let mut lens = [0; N_TRUCKS];
        for pal in self.palets.iter().cloned() {
            let mut to_truck = self.rng.next_u64() as usize % N_TRUCKS;
            let mut last = lens[to_truck];
            while last >= TRUCK_CAP {
                to_truck = self.rng.next_u64() as usize % N_TRUCKS;
                last = lens[to_truck];
            }
            new_sol[to_truck][last] = pal;
            lens[to_truck] += 1;
        }
        new_sol
    }
}
