#[allow(unused)]
use practica_one::rng::{get_time_usize, next_f64_range, RNG, SEED};
use practica_one::{algo::*, logger::reset_log};

const ITER_PER_ALGO: usize = 5;
const ALGOS: [SearchType; 3] = [SearchType::LS, SearchType::SA, SearchType::TS];

#[allow(unused)]
fn main() {
    let cities = read_palets(PALETS_PATH);
    let distances = read_distances(DISTANCE_PATH);

    reset_log();
    const SEEDS: [usize; 5] = [123456, 654321, 1, 2, 3];

    // println!("\n\nRandom Search: ");
    // let mut rng = SmallRng::seed_from_u64(SEED as u64);
    // measure_time(|| {
    //     for i in 0..ITER_PER_ALGO {
    //         RNG::set_new_seed(SEEDS[i]);
    //         let mut search = RandomSearch::new(&distances, cities.clone());
    //         search.run();
    //     }
    // });

    println!("\n\n Memetico: ");
    measure_time(|| {
        for i in 0..ITER_PER_ALGO {
            RNG::set_new_seed(SEEDS[i]);
            let name = format!("{:?}_{}", LEVEL, SEEDS[i]);
            let mut search = Memetic::new(&distances, &cities, &name);
            let s = search.run();
            // println!("{}", cost(&distances, &s));
        }
    });

    return ();

    println!("\n\n Genetico: ");
    measure_time(|| {
        for i in 0..ITER_PER_ALGO {
            RNG::set_new_seed(SEEDS[i]);
            let mut search = Genetic::new(&distances, &cities);
            let s = search.run();
            // println!("{}", cost(&distances, &s));
        }
    });

    return ();

    println!("\n\nLocal Search: ");
    measure_time(|| {
        for i in 0..ITER_PER_ALGO {
            RNG::set_new_seed(SEEDS[i]);
            let search = LocalSearchBF::new(&distances, &cities);
            let s = search.run();
            println!("{}", cost(&distances, &s));
        }
    });

    println!("\n\nSimulated Annealing: ");
    measure_time(|| {
        for i in 0..ITER_PER_ALGO {
            RNG::set_new_seed(SEEDS[i]);
            let search = SimulatedAnnealing::new(&distances, &cities);
            let s = search.run();
            println!("{}", cost(&distances, &s));
        }
    });

    println!("\n\nTabu Search: ");
    measure_time(|| {
        for i in 0..ITER_PER_ALGO {
            RNG::set_new_seed(SEEDS[i]);
            let mut search = TabuSearch::new(&distances, &cities);
            let s = search.run();
            println!("{}", cost(&distances, &s));
        }
    });

    println!("\n\nGreedy: ");
    measure_time(|| {
        let search = Greedy::new(&distances, cities.clone());
        let c = search.run();
        println!("[Greedy] Coste: {:?}", cost(&distances, &c));
    });

    println!("\n\n ==================================================== \n\n");
    println!("\n\n                     PRACTICA 2                       \n\n");
    println!("\n\n ==================================================== \n\n");

    println!("\n\nILS Search: ");
    measure_time(|| {
        for algo in ALGOS {
            for i in 0..1 {
                RNG::set_new_seed(SEEDS[i]);
                let search = Ils::new(&distances, &cities);
                let r = search.run(algo);
            }
            println!();
        }
    });

    println!("\n\nGrasp Search: ");
    measure_time(|| {
        for algo in ALGOS {
            for i in 0..1 {
                RNG::set_new_seed(SEEDS[i]);
                let search = Grasp::new(&distances, &cities);
                let r = search.run(algo);
            }
            println!();
        }
    });

    /*
    let mut m_sol = [
        [22, 12, 33, 10, 11, 12, 12, 11, 17, 10, 12, 49, 17, 0],
        [43, 11, 26, 1, 38, 2, 38, 20, 39, 20, 2, 26, 38, 25],
        [22, 12, 11, 42, 14, 12, 35, 35, 12, 17, 17, 12, 49, 21],
        [22, 10, 24, 4, 4, 14, 24, 7, 4, 48, 7, 6, 24, 31],
        [22, 37, 37, 2, 45, 1, 45, 37, 2, 32, 5, 32, 32, 41],
        [16, 7, 5, 21, 47, 1, 38, 19, 47, 19, 38, 7, 43, 25],
        [39, 20, 16, 39, 7, 20, 14, 3, 49, 37, 22, 49, 49, 8],
        [30, 45, 45, 46, 34, 5, 8, 27, 27, 27, 8, 27, 34, 23],
        [43, 18, 33, 10, 10, 11, 17, 45, 2, 21, 10, 11, 10, 23],
        [19, 19, 34, 4, 36, 36, 36, 36, 19, 36, 19, 4, 22, 23],
        [28, 46, 9, 4, 46, 35, 46, 46, 18, 4, 35, 46, 15, 44],
        [13, 40, 48, 10, 40, 40, 11, 10, 17, 45, 1, 43, 40, 35],
    ];
    */
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
