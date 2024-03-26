pub mod algo;
pub use algo::*;

#[allow(unused)]
pub mod rng {
    use std::cell::OnceCell;
    use std::iter::Cycle;
    use std::sync::atomic::AtomicIsize;
    use std::sync::atomic::Ordering::Relaxed;

    use rand::rngs::SmallRng;
    use rand::{random, Rng, RngCore, SeedableRng};

    pub const SEED: usize = 334;
    const MASK: isize = 29871152;
    const PRIME: isize = 65539;
    static CURRENT: AtomicIsize = AtomicIsize::new(SEED as isize);

    static mut SRNG: Option<SmallRng> = None;

    /// Do not use this in multi thread
    /// Or at least dont say you are using it.
    pub struct RNG;

    use rand::distributions::{DistIter, Standard};

    type DD = rand::distributions::Uniform<usize>;
    impl RNG {
        pub fn set_new_seed(seed: usize) {
            unsafe {
                SRNG = Some(SmallRng::seed_from_u64(seed as u64));
            }
        }

        pub fn cicle() -> DistIter<DD, &'static mut SmallRng, usize> {
            unsafe {
                match SRNG.as_mut() {
                    Some(r) => {
                        r.sample_iter(rand::distributions::Uniform::<usize>::new(0, 10000000000))
                    }
                    None => {
                        SRNG = Some(SmallRng::seed_from_u64(33));
                        let r = SRNG.as_mut().unwrap();
                        r.sample_iter(rand::distributions::Uniform::<usize>::new(0, 10000000000))
                    }
                }
            }
        }

        pub fn next() -> usize {
            let x = unsafe {
                match SRNG.as_mut() {
                    Some(r) => Some(r.next_u64() as usize),
                    None => {
                        SRNG = Some(SmallRng::seed_from_u64(33));
                        None
                    }
                }
            };

            x.unwrap_or(unsafe { SRNG.as_mut().map(|s| s.next_u64() as usize).unwrap() })
        }

        pub fn next_f64() -> f64 {
            let x = unsafe {
                match SRNG.as_mut() {
                    Some(r) => Some(r.gen()),
                    None => {
                        SRNG = Some(SmallRng::seed_from_u64(33));
                        None
                    }
                }
            };
            x.unwrap_or(unsafe { SRNG.as_mut().map(|s| s.gen()).unwrap() })
        }
    }

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

        let mut next = CURRENT.load(Relaxed);
        next = (next + 1) * PRIME + (MASK ^ (next << 3) * PRIME) - (MASK ^ (next >> 2));
        CURRENT.store(next, Relaxed);
        next as usize
    }

    pub fn next_f64_range(min: f64, max: f64) -> f64 {
        let x = next_usize() as f64 / (usize::MAX as f64);
        min + (max - min) * (x)
    }
}
