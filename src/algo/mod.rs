#![allow(unused_imports)]

mod random_search;
pub use random_search::RandomSearch;

mod local_search_bf;
pub use local_search_bf::LocalSearch as LocalSearchBF;

mod simulated_annealing;
pub use simulated_annealing::SimulatedAnnealing;

mod tabu_search;
pub use tabu_search::TabuSearch;

mod greedy_exp;
pub use greedy_exp::GreedyExp;

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

const N_EVAL: usize = N * 5_000;

const TRUCK_CAP: usize = 14;
const ANN_CONST: f64 = 0.95;

/// Change Between Trucks
const CBT: bool = true;

#[allow(dead_code)]
const MAX_TRIES: usize = 500;

/// N x N Matriz
type Costs = Vec<Vec<usize>>;
type Palets = Vec<u16>;
type Trucks = [[u16; TRUCK_CAP]; N_TRUCKS];

pub fn read_palets(file: &str) -> Palets {
    let content = read_to_string(file);
    let content = content.expect("Could not read the palets file");
    content
        .lines()
        .map(|l| l.trim().parse().expect("Couldnt parse the value"))
        .collect()
}

pub fn read_distances(file: &str) -> Costs {
    read_to_string(file)
        .expect("Could not read the distances file")
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|l| l.trim().parse().expect("Couldnt parse the value"))
                .collect::<Vec<usize>>()
        })
        .collect::<Costs>()
}

fn gen_neighbour(sol: &Trucks, change_palets: bool) -> Trucks {
    let mut nb = two_op_in_truck(sol);
    if change_palets {
        two_op_between_trucks(&mut nb);
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
    let mut nb = *sol;
    if change_palets {
        let aux = nb[truck][a];
        nb[truck][a] = nb[truck_b][b];
        nb[truck_b][b] = aux;
    } else {
        nb[truck].swap(a, b)
    }
    nb
}

fn two_op_in_truck(sol: &Trucks) -> Trucks {
    let mut sol = *sol;
    let truck = rng::next_usize() % N_TRUCKS;

    let from = rng::next_usize() % TRUCK_CAP;
    let mut to = rng::next_usize() % TRUCK_CAP;
    while to == from {
        to = rng::next_usize() % TRUCK_CAP;
    }

    sol[truck].swap(to, from);
    sol
}

fn two_op_between_trucks(new_sol: &mut Trucks) -> &mut Trucks {
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

pub fn cost(cost_mat: &Costs, sol: &Trucks) -> usize {
    let mut cost = 0;
    let mut visited;
    let mut actual_city;

    for truck in sol.iter() {
        visited = [false; N];
        actual_city = 0;
        visited[0] = true;
        cost += truck
            .iter()
            .map(|e| (e - 1) as usize)
            .filter_map(|x| {
                (!visited[x]).then(|| {
                    let c = cost_mat[actual_city][x];
                    visited[x] = true;
                    actual_city = x;
                    c
                })
            })
            .sum::<usize>()
            + cost_mat[actual_city][0];
    }

    cost
}

fn gen_sol(palets: &Palets) -> Trucks {
    let mut new_sol = Trucks::default();
    let mut lens = [0; N_TRUCKS];
    for pal in palets.iter().cloned() {
        let mut to_truck = rng::next_usize() % N_TRUCKS;
        let mut last = lens[to_truck];
        while last >= TRUCK_CAP {
            to_truck = rng::next_usize() % N_TRUCKS;
            last = lens[to_truck];
        }
        new_sol[to_truck][last] = pal;
        lens[to_truck] += 1;
    }
    new_sol
}
