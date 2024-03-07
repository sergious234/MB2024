#[allow(unused)]
mod algo {
    use super::rng::next_usize;
    use std::fs::read_to_string;

    pub fn read_palets(file: &str) -> [usize; N_PALETS] {
        let content = read_to_string(file);
        let content = content.expect("No se pudo leer el fichero de destinos");
        let mut trucks: [usize; N_PALETS] = [0; N_PALETS];

        let mut i = 0;
        for n in content.lines() {
            let truck: usize = n.trim().parse().expect("Couldnt parse the value");
            trucks[i] = truck;
            i += 1;
        }
        trucks
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

    const N_EVAL: usize = 1000;
    const N_PALETS: usize = 84;
    const N: usize = 25;
    const N_TRUCKS: usize = 6;

    type Costs = [[usize; N]; N];

    pub struct RandomSearch {
        cost_mat: Costs,
        palets: [usize; N_PALETS],
        n_trucks: usize,
    }

    impl RandomSearch {
        pub fn new(cost_mat: Costs, palets: [usize; N_PALETS]) -> Self {
            RandomSearch {
                cost_mat,
                palets,
                n_trucks: N,
            }
        }

        pub fn run(&mut self) {}
    }
}

fn main() {
    use algo::*;

    let cities = read_palets("../data/destinos_palets_84.txt");
    let distances = read_distances("../data/matriz_distancias_25.txt");
    let mut search = RandomSearch::new(distances, cities);
    search.run();
}

#[allow(unused)]
mod rng {
    use std::sync::atomic::AtomicIsize;

    const SEED: usize = 33333;
    const MASK: isize = 29871152;
    const PRIME: isize = 65539;
    static CURRENT: AtomicIsize = AtomicIsize::new(SEED as isize);

    pub fn next_usize() -> usize {
        let mut next = CURRENT.load(std::sync::atomic::Ordering::Relaxed);
        next = next * PRIME + (MASK ^ (next << 3)) - (MASK ^ (next >> 2));
        CURRENT.store(next, std::sync::atomic::Ordering::Relaxed);
        next as usize
    }
}
