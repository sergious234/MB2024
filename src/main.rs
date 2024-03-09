mod algo;

#[allow(unused)]
fn main() {
    use algo::*;

    let cities = read_palets(PALETS_PATH);
    let distances = read_distances(DISTANCE_PATH);

    println!("\n\nRandom Search: ");
    rng::set_new_seed(333);
    for _i in 0..5 {
        let search = RandomSearch::new(&distances, cities.clone());
        search.run();
    }

    println!("\n\nLocal Search: ");
    rng::set_new_seed(333);
    for _i in 0..5 {
        let search = LocalSearchBF::new(&distances, cities.clone());
        search.run();
    }

    println!("\n\nSimulated Annealing: ");
    rng::set_new_seed(333);
    for _i in 0..5 {
        let search = SimulatedAnnealing::new(&distances, cities.clone());
        search.run();
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
        let mut next = CURRENT.load(Relaxed);
        next = (next + 1) * PRIME + (MASK ^ (next << 3) * PRIME) - (MASK ^ (next >> 2));
        CURRENT.store(next, Relaxed);
        next as usize
    }

    pub fn next_usize_range(min: usize, max: usize) -> usize {
        rand::random::<usize>() % (max - min + 1) + min
    }
}
