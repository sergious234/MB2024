use crate::rng::get_time_usize;

mod algo;

const ITER_PER_ALGO: usize = 5;

#[allow(unused)]
fn main() {
    use algo::*;

    let cities = read_palets(PALETS_PATH);
    let distances = read_distances(DISTANCE_PATH);

    println!("\n\nRandom Search: ");
    rng::set_new_seed(get_time_usize());
    measure_time(|| {
        for _i in 0..ITER_PER_ALGO {
            let search = RandomSearch::new(&distances, cities.clone());
            search.run();
        }
    });
    println!("\n\nLocal Search: ");
    rng::set_new_seed(get_time_usize());

    measure_time(|| {
    for _i in 0..ITER_PER_ALGO {
        let search = LocalSearchBF::new(&distances, cities.clone());
        search.run();
    }});

    println!("\n\nSimulated Annealing: ");
    rng::set_new_seed(get_time_usize());
    measure_time(|| {
    for _i in 0..ITER_PER_ALGO {
        let search = SimulatedAnnealing::new(&distances, cities.clone());
        search.run();
    }});

    println!("\n\nTabu Search: ");
    rng::set_new_seed(get_time_usize());
    measure_time(|| {
    for _i in 0..ITER_PER_ALGO {
        let mut search = TabuSearch::new(&distances, cities.clone());
        search.run();
    }});

    println!("\n\nGreedy: ");
    measure_time(|| {
    for _i in 0..ITER_PER_ALGO {
        let search = Greedy::new(&distances, cities.clone());
        search.run();
    }});
}

fn measure_time(fun: impl FnOnce() -> ()) {
    let current = std::time::Instant::now();
    fun();
    let end = std::time::Instant::now();
    println!(
        "{}ms per search",
        end.duration_since(current).as_millis() / ITER_PER_ALGO as u128
    );
}

#[allow(unused)]
mod rng {
    use std::sync::atomic::AtomicIsize;
    use std::sync::atomic::Ordering::Relaxed;

    use rand::random;

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
        // NOTE: Homemade random number generator.
        /*
        let mut next = CURRENT.load(Relaxed);
        next = (next + 1) * PRIME + (MASK ^ (next << 3) * PRIME) - (MASK ^ (next >> 2));
        CURRENT.store(next, Relaxed);
        next as usize
        */
        rand::random()
    }

    pub fn next_f64_range(min: f64, max: f64) -> f64 {
        rand::random::<f64>() % max
    }
}
