use practica_one::rng;
use rand::{rngs::SmallRng, SeedableRng};

use practica_one::algo::*;
#[allow(unused)]
use practica_one::rng::{get_time_usize, next_f64_range, SEED};
const ITER_PER_ALGO: usize = 5;

#[allow(unused)]
fn main() {
    let cities = read_palets(PALETS_PATH);
    let distances = read_distances(DISTANCE_PATH);

    println!("\n\nRandom Search: ");
    let mut rng = SmallRng::seed_from_u64(SEED as u64);
    measure_time(|| {
        for _i in 0..ITER_PER_ALGO {
            let mut search = RandomSearch::new(&distances, cities.clone(), &mut rng);
            search.run();
        }
    });

    println!("\n\nLocal Search: ");
    rng::set_new_seed(SEED);
    measure_time(|| {
        for _i in 0..ITER_PER_ALGO {
            let search = LocalSearchBF::new(&distances, cities.clone());
            let r = search.run();
        }
    });

    println!("\n\nSimulated Annealing: ");
    rng::set_new_seed(SEED);
    measure_time(|| {
        for _i in 0..ITER_PER_ALGO {
            let search = SimulatedAnnealing::new(&distances, cities.clone());
            search.run();
        }
    });

    println!("\n\nTabu Search: ");
    rng::set_new_seed(SEED);
    measure_time(|| {
        for _i in 0..ITER_PER_ALGO {
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
