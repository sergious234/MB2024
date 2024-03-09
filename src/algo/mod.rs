#![allow(unused_imports)]

mod random_search;
pub use random_search::RandomSearch;

mod local_search_bf;
pub use local_search_bf::LocalSearch as LocalSearchBF;

mod simulated_annealing;
pub use simulated_annealing::SimulatedAnnealing;

mod tabu_search;
pub use tabu_search::TabuSearch;

use super::rng;
use std::{collections::HashSet, fs::read_to_string, usize};

pub const PALETS_PATH: &str = "../data/destinos_palets_84.txt";
pub const DISTANCE_PATH: &str = "../data/matriz_distancias_25.txt";

const N_EVAL: usize = 100_000;

#[allow(unused)]
const N_PALETS: usize = 84;

/// Number of cities
const N: usize = 25;
const N_TRUCKS: usize = 6;
const TRUCK_CAP: usize = 14;
const ANN_CONST: f64 = 0.99;

// Change Between Trucks
const CBT: bool = true;
const MAX_COM: usize = if CBT {
    (N_TRUCKS * N_TRUCKS) * ((TRUCK_CAP * (TRUCK_CAP - 1)) / 2)
} else {
    (TRUCK_CAP * (TRUCK_CAP - 1)) / 2
};

const MAX_TRIES: usize = 10;

/// N x N Matriz
type Costs = Vec<Vec<usize>>;
type Palets = Vec<usize>;
type Trucks = [Vec<usize>; N_TRUCKS];

pub fn read_palets(file: &str) -> Palets {
    let content = read_to_string(file);
    let content = content.expect("Could not read the palets file");
    content
        .lines()
        .map(|l| l.trim().parse().expect("Couldnt parse the value"))
        .collect()
}

pub fn read_distances(file: &str) -> Costs {
    let content = read_to_string(file);
    let content: Vec<Vec<usize>> = content
        .expect("Could not read the distances file")
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|l| l.trim().parse().expect("Couldnt parse the value"))
                .collect::<Vec<usize>>()
        })
        .collect();

    let mut costs: Costs = Costs::default();

    for i in 0..N {
        costs.push(Vec::with_capacity(N));
        for j in 0..N {
            costs[i].push(content[i][j]);
        }
    }
    costs
}

fn gen_neighbour(sol: &Trucks, change_palets: bool) -> Trucks {
    let mut nb = two_op_in_truck(sol);
    if change_palets {
        nb = two_op_within_truck(&nb);
    }
    nb
}

fn two_op_in_truck(sol: &Trucks) -> Trucks {
    let mut sol = sol.clone();
    let truck = rng::next_usize() % N_TRUCKS;

    let from = rng::next_usize() % TRUCK_CAP;
    let mut to = rng::next_usize() % TRUCK_CAP;
    while to == from {
        to = rng::next_usize() % TRUCK_CAP;
    }

    let aux = sol[truck][to];
    sol[truck][to] = sol[truck][from];
    sol[truck][from] = aux;
    sol
}

fn two_op_within_truck(sol: &Trucks) -> Trucks {
    let mut new_sol = sol.clone();

    let from_truck = rng::next_usize() % N_TRUCKS;
    let mut to_truck = rng::next_usize() % N_TRUCKS;
    while to_truck == from_truck {
        to_truck = rng::next_usize() % N_TRUCKS;
    }

    let from = rng::next_usize() % TRUCK_CAP;
    let mut to = rng::next_usize() % TRUCK_CAP;
    while to == from {
        to = rng::next_usize() % TRUCK_CAP;
    }

    let aux = new_sol[to_truck][to];
    new_sol[to_truck][to] = new_sol[from_truck][from];
    new_sol[from_truck][from] = aux;

    new_sol
}
