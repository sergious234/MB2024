#![allow(unused_imports)]

mod random_search;
pub use random_search::RandomSearch;

mod local_search_bf;
pub use local_search_bf::LocalSearch as LocalSearchBF;

mod simulated_annealing;
pub use simulated_annealing::SimulatedAnnealing;

mod tabu_search;
pub use tabu_search::TabuSearch;

mod greedy;
pub use greedy::Greedy;

use super::rng;
use std::{collections::HashSet, fs::read_to_string, usize};

#[allow(dead_code)]
enum Level {
    Small,
    Medium,
    Large,
}

const LEVEL: Level = Level::Large;

pub const PALETS_PATH: &str = match LEVEL {
    Level::Small => "../data/destinos_palets_84.txt",
    Level::Medium => "../data/destinos_palets_126.txt",
    Level::Large => "../data/destinos_palets_168.txt",
};
pub const DISTANCE_PATH: &str = match LEVEL {
    Level::Small => "../data/matriz_distancias_25.txt",
    Level::Medium => "../data/matriz_distancias_38.txt",
    Level::Large => "../data/matriz_distancias_50.txt",
};

#[allow(unused)]
const N_PALETS: usize = match LEVEL {
    Level::Small => 84,
    Level::Medium => 126,
    Level::Large => 168,
};

/// Number of cities
const N: usize = match LEVEL {
    Level::Small => 25,
    Level::Medium => 38,
    Level::Large => 50,
};

const N_TRUCKS: usize = match LEVEL {
    Level::Small => 6,
    Level::Medium => 9,
    Level::Large => 12,
};

const N_EVAL: usize = N * 1000;

const TRUCK_CAP: usize = 14;
const ANN_CONST: f64 = 0.99;

/// Change Between Trucks
const CBT: bool = true;

#[allow(dead_code)]
const MAX_COM: usize = if CBT {
    (N_TRUCKS * N_TRUCKS) * ((TRUCK_CAP * (TRUCK_CAP - 1)) / 2)
} else {
    (TRUCK_CAP * (TRUCK_CAP - 1)) / 2
};

const MAX_TRIES: usize = 500;

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
        nb = two_op_between_trucks(&nb);
    }
    nb
}

#[allow(dead_code)]
fn gen_neighbour_2(
    sol: &Trucks,
    change_palets: bool,
    truck: usize,
    truck_b: usize,
    a: usize,
    b: usize,
) -> Trucks {
    let mut nb = sol.clone();
    let aux = nb[truck][a];
    nb[truck][a] = nb[truck][b];
    nb[truck][b] = aux;
    if change_palets {
        let aux = nb[truck][a];
        nb[truck][a] = nb[truck_b][b];
        nb[truck_b][b] = aux;
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

fn two_op_between_trucks(sol: &Trucks) -> Trucks {
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

fn cost(cost_mat: &Costs, sol: &Trucks) -> usize {
    let mut cost = 0;

    let mut visited = HashSet::new();
    for truck in sol.iter() {
        let mut actual_city = 0;
        for city in truck.iter().map(|e| e - 1) {
            if visited.insert(city) {
                cost += cost_mat[actual_city][city];
                actual_city = city;
            }
        }
        visited.clear();

        cost += cost_mat[actual_city][0];
    }
    cost
}
