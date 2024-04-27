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

mod grasp;
pub use grasp::Grasp;

mod ils;
pub use ils::Ils;

mod genetic;
pub use genetic::Genetic;

mod memetic;
pub use memetic::Memetic;

use super::rng::RNG;
use std::fs::read_to_string;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Level {
    Small,
    Medium,
    Large,
}

pub const LEVEL: Level = Level::Large;

pub const PALETS_PATH: &str = match LEVEL {
    Level::Small => "data/destinos_palets_84.txt",
    Level::Medium => "data/destinos_palets_126.txt",
    Level::Large => "data/destinos_palets_168.txt",
};
pub const DISTANCE_PATH: &str = match LEVEL {
    Level::Small => "data/matriz_distancias_25.txt",
    Level::Medium => "data/matriz_distancias_38.txt",
    Level::Large => "data/matriz_distancias_50.txt",
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
const ANN_CONST: f64 = 0.9;

/// Change Between Trucks
const CBT: bool = true;

/// N x N Matriz
type Palet = u8;
type Costs = Vec<Vec<usize>>;

type Palets = Vec<Palet>;
type Trucks = [[Palet; TRUCK_CAP]; N_TRUCKS];

#[derive(Debug, Clone, Copy)]
pub enum SearchType {
    LS,
    SA,
    TS,
}

impl SearchType {
    pub fn search(&self, cost_mat: &Costs, palets: &Palets, start: Trucks) -> Trucks {
        match self {
            Self::LS => LocalSearchBF::new(cost_mat, palets).run_with_start(start),
            Self::SA => SimulatedAnnealing::new(cost_mat, palets).run_with_start(start),
            Self::TS => TabuSearch::new(cost_mat, palets).run_with_start::<false>(start),
        }
    }
}

pub fn read_palets(file: &str) -> Palets {
    read_to_string(file)
        .expect("Could not read the palets file")
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
    let mut sol = *sol;

    if change_palets {
        two_op_between_trucks(&mut sol);
    } else {
        two_op_in_truck(&mut sol);
    }
    sol
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
        nb[truck].swap(a, b);
    }
    nb
}

fn two_op_in_truck(sol: &mut Trucks) -> &mut Trucks {
    let from = RNG::next() % TRUCK_CAP;
    let mut to = RNG::next() % TRUCK_CAP;

    let truck = RNG::next() % N_TRUCKS;

    while to == from {
        to = RNG::next() % TRUCK_CAP;
    }

    sol[truck].swap(to, from);
    sol
}

fn two_op_between_trucks(new_sol: &mut Trucks) -> &mut Trucks {
    let from_truck = RNG::next() % N_TRUCKS;
    let mut to_truck = RNG::next() % N_TRUCKS;
    while to_truck == from_truck {
        to_truck = RNG::next() % N_TRUCKS;
    }

    let from = RNG::next() % TRUCK_CAP;
    let mut to = RNG::next() % TRUCK_CAP;
    while to == from {
        to = RNG::next() % TRUCK_CAP;
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

fn gen_sol2(palets: &Palets) -> Trucks {
    let mut new_sol = Trucks::default();

    let mut pals = palets.clone();
    for t in new_sol.iter_mut() {
        let mut last = 0;
        while last < TRUCK_CAP {
            let index = RNG::next() % pals.len();
            let x = pals.remove(index);
            t[last] = x;
            last += 1;
        }
    }
    new_sol
}

fn gen_sol(palets: &Palets) -> Trucks {
    let mut new_sol = Trucks::default();
    let mut lens = [0; N_TRUCKS];
    for pal in palets.iter().cloned() {
        let mut to_truck = RNG::next() % N_TRUCKS;
        let mut last = lens[to_truck];
        while last >= TRUCK_CAP {
            to_truck = RNG::next() % N_TRUCKS;
            last = lens[to_truck];
        }
        new_sol[to_truck][last] = pal;
        lens[to_truck] += 1;
    }
    new_sol
}
