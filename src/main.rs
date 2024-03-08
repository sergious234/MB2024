#[allow(unused)]
mod algo {
    use super::rng;
    use std::{
        collections::{BTreeSet, HashSet},
        fs::read_to_string,
        usize,
    };

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
            for j in 0..N {
                costs[i][j] = content[i][j];
            }
        }
        costs
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

    const N_EVAL: usize = 100_000;
    const N_PALETS: usize = 84;

    // Number of cities
    const N: usize = 25;
    const N_TRUCKS: usize = 6;
    const TRUCK_CAP: usize = 14;
    const ANN_CONST: f64 = 0.99;

    const CBT: bool = true;
    const MAX_COM: usize = if CBT {
        (N_TRUCKS * N_TRUCKS) * ((TRUCK_CAP * (TRUCK_CAP - 1)) / 2)
    } else {
        ((TRUCK_CAP * (TRUCK_CAP - 1)) / 2)
    };

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

        pub fn run(&self) {
            let mut sol = self.gen_sol();
            let mut it = 0;

            let mut best_sol = sol;
            let mut best_cost = self.cost(&best_sol);
            while it < N_EVAL {
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

        pub fn run(&self) {
            let mut actual_sol = self.gen_sol();
            let mut best_sol = actual_sol.clone();
            let mut best_cost = self.cost(&best_sol);

            let mut it = 0;

            let mut visitados = HashSet::new();
            let mut switch = true;
            'main_loop: while it < N_EVAL {
                let mut next_sol = self.gen_neighbour(&actual_sol, switch);
                visitados.clear();
                visitados.insert(next_sol.clone());

                while self.cost(&next_sol) >= best_cost {
                    next_sol = self.gen_neighbour(&actual_sol, switch);
                    while visitados.contains(&next_sol) {
                        next_sol = self.gen_neighbour(&actual_sol, switch);
                    }
                    visitados.insert(next_sol.clone());
                    if visitados.len() >= MAX_COM {
                        break 'main_loop;
                    }
                }

                if CBT {
                    switch != switch;
                }

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
            let mut nb = two_op_in_truck(sol);
            if change_palets {
                nb = two_op_within_truck(&nb);
            }
            nb
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

    pub struct SimulatedAnnealing {
        cost_mat: Costs,
        palets: Palets,
        trucks: Trucks,
    }

    enum AnnealingMech {
        ExponentialDescend,
        BoltzmannCriteria,
        CaucheEscheme,
    }

    impl AnnealingMech {
        pub fn update(&self, temp: usize, alpha_iter: f64) -> usize {
            match self {
                Self::ExponentialDescend => alpha_iter as usize * temp,
                Self::BoltzmannCriteria => {
                    (temp as f64 / (1.0 + alpha_iter.log10())).floor() as usize
                }
                Self::CaucheEscheme => (temp as f64 / (1.0 + alpha_iter)).floor() as usize,
            }
        }
    }

    impl SimulatedAnnealing {
        pub fn new(cost_mat: Costs, palets: Palets) -> Self {
            let mut x: [Vec<usize>; N_TRUCKS] = Default::default();
            for i in 0..N_TRUCKS {
                x[i] = vec![];
            }

            Self {
                cost_mat,
                palets,
                trucks: x,
            }
        }

        pub fn run(&self) {
            let mut best_sol = self.gen_sol();
            let mut best_cost = self.cost(&best_sol);

            let mut visitados = HashSet::new();
            let mut switch = true;
            let lt = (MAX_COM as f64 / 1.3) as usize;

            let init_temp = best_cost;
            let mut temp = init_temp;

            let ann_mech = AnnealingMech::CaucheEscheme;

            let aceptacion = |delta: isize, t: usize| (-(delta as f64) / t as f64).exp() as usize;

            let mut upgrade = true;
            while temp > 0 && upgrade {
                upgrade = false;
                for cont in 0..lt {
                    // Para no repetir candidatos
                    visitados.clear();
                    let mut sol_cand = self.gen_neighbour(&best_sol, CBT);
                    while visitados.contains(&sol_cand) {
                        sol_cand = self.gen_neighbour(&best_sol, CBT);
                    }
                    visitados.insert(sol_cand.clone());

                    let cost_cand = self.cost(&sol_cand);

                    // Se calcula al reves debido a que buscamos un minimo no un maximo.
                    let delta_cost = cost_cand as isize - best_cost as isize;

                    if delta_cost < 0 || rng::next_usize_range(0, 1) < aceptacion(delta_cost, temp)
                    {
                        best_cost = cost_cand;
                        best_sol = sol_cand;

                        upgrade = true;
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
            let mut nb = two_op_in_truck(sol);
            if change_palets {
                nb = two_op_within_truck(&nb);
            }
            nb
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

    println!("\n\nRandom Search: ");
    for _i in 0..5 {
        let search = RandomSearch::new(distances, cities.clone());
        search.run();
        rng::set_new_seed(rng::get_time_usize());
    }

    println!("\n\nSimulated Annealing: ");
    for _i in 0..5 {
        let search = SimulatedAnnealing::new(distances, cities.clone());
        search.run();
        rng::set_new_seed(rng::get_time_usize());
    }

    println!("\n\nLocal Search: ");
    for _i in 0..5 {
        let search = LocalSearch::new(distances, cities.clone());
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

    use rand;
    pub fn next_usize() -> usize {
        /*
        let mut next = CURRENT.load(Relaxed);
        next = (next + 1) * PRIME + (MASK ^ (next << 3) * PRIME) - (MASK ^ (next >> 2));
        CURRENT.store(next, Relaxed);
        next as usize;
        */

        rand::random()
    }

    pub fn next_usize_range(min: usize, max: usize) -> usize {
        rand::random::<usize>() % (max - min + 1) + min
    }
}
