#[allow(unused)]
mod algo {
    use super::rng;
    use std::{collections::{BTreeSet, HashSet}, fs::read_to_string};

    pub fn read_palets(file: &str) -> Palets {
        let content = read_to_string(file);
        let content = content.expect("No se pudo leer el fichero de destinos");
        content
            .lines()
            .map(|l| l.trim().parse().expect("Couldnt parse the value"))
            .collect()
    }

    pub fn read_distances(file: &str) -> Costs {
        let content = read_to_string(file);
        let content: Vec<Vec<usize>> = content
            .expect("No se pudo leer el fichero de distancias")
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .map(|l| l.trim().parse().expect("Coudlnt parse the value"))
                    .collect::<Vec<usize>>()
            })
            .collect();

        let mut costs: Costs = Costs::default();

        for i in 0..N {
            for j in 0..N {
                costs[i][j] = content[i][j];
            }
        }
        costs
    }

    const N_EVAL: usize = 1_000;
    const N_PALETS: usize = 84;
    const N: usize = 25;
    const N_TRUCKS: usize = 6;
    const TRUCK_CAP: usize = 14;

    type Costs = [[usize; N]; N];
    type Palets = Vec<usize>;
    type Trucks = [Vec<usize>; N_TRUCKS];

    pub struct RandomSearch {
        cost_mat: Costs,
        palets: Palets,
        trucks: Trucks,
    }

    impl RandomSearch {
        pub fn new(cost_mat: Costs, palets: Palets) -> Self {
            let mut x: [Vec<usize>; N_TRUCKS] = Default::default();
            for i in 0..N_TRUCKS {
                x[i] = vec![];
            }

            RandomSearch {
                cost_mat,
                palets,
                trucks: x,
            }
        }

        pub fn run(&mut self) {
            let mut sol = self.gen_sol();
            let mut it = 0;

            let mut best_sol = sol;
            let mut best_cost = self.cost(&best_sol);
            while it < N_EVAL || false {
                sol = self.gen_sol();
                let new_cost = self.cost(&sol);

                if new_cost < best_cost {
                    best_sol = sol;
                    best_cost = new_cost;
                }

                it += 1;
            }

            for (i, t) in best_sol.iter().enumerate() {
                println!("Truck {}: {:?}", i, t);
            }
            println!("Coste: {}", best_cost)
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

    pub struct LocalSearch {
        cost_mat: Costs,
        palets: Palets,
        trucks: Trucks,
    }

    impl LocalSearch {
        pub fn new(cost_mat: Costs, palets: Palets) -> Self {
            let mut x: [Vec<usize>; N_TRUCKS] = Default::default();
            for i in 0..N_TRUCKS {
                x[i] = vec![];
            }

            LocalSearch {
                cost_mat,
                palets,
                trucks: x,
            }
        }

        pub fn run(&mut self) {
            let mut actual_sol = self.gen_sol();
            let mut best_sol = actual_sol.clone();
            let mut best_cost = self.cost(&best_sol);

            let mut it = 0;


            const CBT: bool = true;
            const MAX_COM: usize = (N_TRUCKS*N_TRUCKS)*((TRUCK_CAP*(TRUCK_CAP-1))/2);

            let mut visitados = HashSet::new();
            'main_loop: while it < N_EVAL {
                let mut next_sol = self.gen_neighbour(&actual_sol, CBT);
                visitados.clear();
                visitados.insert(next_sol.clone());

                while self.cost(&next_sol) >= best_cost {

                    next_sol = self.gen_neighbour(&actual_sol, CBT);
                    while visitados.contains(&next_sol) {
                        next_sol = self.gen_neighbour(&actual_sol, CBT);
                    }

                    visitados.insert(next_sol.clone());

                    if visitados.len() >= MAX_COM {
                        break 'main_loop;
                    }                 }

                best_sol = actual_sol.clone();
                best_cost = self.cost(&best_sol);
                actual_sol = next_sol;
                it += 1;
            }

            for (i, t) in best_sol.iter().enumerate() {
                println!("Truck {}: {:?}", i, t);
            }
            println!("Coste: {}", best_cost)
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

        fn gen_neighbour(&self, sol: &Trucks, change_palets: bool) -> Trucks {
            let mut nb = Self::two_op_in_truck(sol);
            if change_palets {
                nb = Self::two_op_within_truck(&nb);
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
}

fn main() {
    use algo::*;

    let cities = read_palets("../data/destinos_palets_84.txt");
    let distances = read_distances("../data/matriz_distancias_25.txt");
    for _i in 0..5 {
        let mut search = LocalSearch::new(distances, cities.clone());
        search.run();
        rng::set_new_seed(rng::get_time_usize());
    }
}

#[allow(unused)]
mod rng {
    use std::sync::atomic::AtomicIsize;
    use std::sync::atomic::Ordering::Relaxed;

    const SEED: usize = 333;
    const MASK: isize = 29871152;
    const PRIME: isize = 65539;
    static CURRENT: AtomicIsize = AtomicIsize::new(SEED as isize);

    pub fn get_time_usize() -> usize {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Could not get systime")
            .as_secs() as usize
    }

    pub fn set_new_seed(new_seed: usize) {
        CURRENT.store(new_seed as isize, Relaxed);
    }

    pub fn next_usize() -> usize {
        use rand;

        /*
        let mut next = CURRENT.load(Relaxed);
        next = (next + 1) * PRIME + (MASK ^ (next << 3) * PRIME) - (MASK ^ (next >> 2));
        CURRENT.store(next, Relaxed);
        next as usize;
        */

        rand::random()

        
    }
}
