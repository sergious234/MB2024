use practica_one::rng;
use rand::{rngs::SmallRng, SeedableRng};

use practica_one::algo::*;
#[allow(unused)]
use practica_one::rng::{get_time_usize, next_f64_range, RNG, SEED};
const ITER_PER_ALGO: usize = 5;

#[allow(unused)]
fn main() {
    let cities = read_palets(PALETS_PATH);
    let distances = read_distances(DISTANCE_PATH);

    const SEEDS: [usize; 5] = [123456, 654321, 1, 2, 3];

    println!("\n\nRandom Search: ");
    let mut rng = SmallRng::seed_from_u64(SEED as u64);
    measure_time(|| {
        for i in 0..ITER_PER_ALGO {
            RNG::set_new_seed(SEEDS[i]);
            let mut search = RandomSearch::new(&distances, cities.clone());
            search.run();
        }
    });

    println!("\n\nLocal Search: ");
    measure_time(|| {
        for i in 0..ITER_PER_ALGO {
            RNG::set_new_seed(SEEDS[i]);
            let search = LocalSearchBF::new(&distances, cities.clone());
            let r = search.run();
        }
    });

    println!("\n\nSimulated Annealing: ");
    measure_time(|| {
        for i in 0..ITER_PER_ALGO {
            RNG::set_new_seed(SEEDS[i]);
            let search = SimulatedAnnealing::new(&distances, cities.clone());
            search.run();
        }
    });

    println!("\n\nTabu Search: ");
    measure_time(|| {
        for i in 0..ITER_PER_ALGO {
            RNG::set_new_seed(SEEDS[i]);
            let mut search = TabuSearch::new(&distances, cities.clone());
            search.run();
        }
    });

    println!("\n\nGreedy: ");
    measure_time(|| {
        let search = Greedy::new(&distances, cities.clone());
        let c = search.run();
        println!("[Greedy] Coste: {:?}", cost(&distances, &c));
    });
}

fn measure_time(fun: impl FnOnce()) {
    let current = std::time::Instant::now();
    fun();
    let end = std::time::Instant::now();
    println!(
        "{}ms per search",
        end.duration_since(current).as_millis() / ITER_PER_ALGO as u128
    );
}
