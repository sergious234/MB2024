pub mod algo;
pub use algo::*;

pub mod logger {
    use std::{
        fs::File,
        io::{BufWriter, Write},
    };

    pub struct DataLogger {
        buff: BufWriter<File>,
        name: String,
    }

    impl DataLogger {
        pub fn new(name: &str) -> Self {
            let file = File::options()
                .append(true)
                .write(true)
                .create(true)
                .open("/home/sergio/Projects/MB/data.txt")
                .expect("Error");

            let buff = BufWriter::new(file);
            Self {
                buff,
                name: name.to_owned(),
            }
        }

        pub fn log(&mut self, it: usize, cost: usize) {
            self.buff
                .write_fmt(format_args!("\"{}\" {} {}\n", self.name, it, cost))
                .expect("Error at writing");
        }
    }

    pub fn reset_log() {
        File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open("/home/sergio/Projects/MB/data.txt")
            .expect("Error");
    }

    pub fn log(it: usize, cost: usize) {
        let file = File::options()
            .append(true)
            .write(true)
            .create(true)
            .open("/home/sergio/Projects/MB/data.txt")
            .expect("Error");

        let mut buff = BufWriter::new(file);

        buff.write_fmt(format_args!("{} {}\n", it, cost))
            .expect("Error at writing");
    }
}

#[allow(unused)]
pub mod rng {
    use std::cell::OnceCell;
    use std::iter::Cycle;
    use std::sync::atomic::Ordering::Relaxed;
    use std::sync::atomic::{AtomicBool, AtomicIsize};

    use rand::rngs::SmallRng;
    use rand::{random, Rng, RngCore, SeedableRng};

    pub const SEED: usize = 334;
    const MASK: isize = 29871152;
    const PRIME: isize = 65539;
    static CURRENT: AtomicIsize = AtomicIsize::new(SEED as isize);

    static mut SRNG: Option<SmallRng> = None;

    static FREE: bool = true;
    static LOCKED: bool = false;
    static mut LOCK: AtomicBool = AtomicBool::new(FREE);

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
                let val = match SRNG.as_mut() {
                    Some(r) => Some(r.next_u64() as usize),
                    None => {
                        SRNG = Some(SmallRng::seed_from_u64(33));
                        None
                    }
                };

                val
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

    impl RngCore for RNG {
        fn next_u32(&mut self) -> u32 {
            RNG::next() as u32
        }

        fn next_u64(&mut self) -> u64 {
            RNG::next() as u64
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
            unimplemented!("no")
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            unimplemented!("no")
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
