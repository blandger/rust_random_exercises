// CPU Cache line comparison example
#![allow(dead_code)]
use std::sync::atomic::AtomicU64;

/// Struct is BAD for CPU cache line because those fields are placed one after another.
#[derive(Debug)]
struct NonCacheLinedData {
    a: AtomicU64,
    b: AtomicU64,
}

/// Struct is GOOD for CPU cache line because those fields are separated by _padding between cache lines.
#[derive(Debug)]
#[repr(align(64))]
struct CacheLineWedged {
    a: AtomicU64,
    _padding: [u64; 7], // separates two cache lines
    b: AtomicU64,
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::Ordering;
    use std::time::Instant;
    use super::*;

    const ITERATIONS: usize = 1_000_000;
    #[test]
    fn test_tightened_lines() {
        let tight = Arc::new(NonCacheLinedData {
            a: AtomicU64::new(0),
            b: AtomicU64::new(0),
        });

        let start = Instant::now();
        let t1 = {
            let tight = tight.clone();
            std::thread::spawn(move || {
                for _ in 0..ITERATIONS {
                    // updating A field
                    tight.a.fetch_add(1, Ordering::Relaxed);
                }
            })
        };
        let t2 = {
            let tight = tight.clone();
            std::thread::spawn(move || {
                for _ in 0..ITERATIONS {
                    // updating B field
                    tight.b.fetch_add(1, Ordering::Relaxed);
                }
            })
        };
        t1.join().unwrap();
        t2.join().unwrap();
        println!("Tight (No Wedge):   {:.2?}", start.elapsed());
        println!("{:?}", &tight);
    }


    #[test]
    fn test_padded_lines() {
        let wedged = Arc::new(CacheLineWedged {
            a: AtomicU64::new(0),
            _padding: [0; 7],
            b: AtomicU64::new(0),
        });

        let start = Instant::now();
        let w1 = {
            let wedged = wedged.clone();
            std::thread::spawn(move || {
                for _ in 0..ITERATIONS {
                    // updating A field
                    wedged.a.fetch_add(1, Ordering::Relaxed);
                }
            })
        };
        let w2 = {
            let wedged = wedged.clone();
            std::thread::spawn(move || {
                for _ in 0..ITERATIONS {
                    // updating B field
                    wedged.b.fetch_add(1, Ordering::Relaxed);
                }
            })
        };
        w1.join().unwrap();
        w2.join().unwrap();
        println!("Wedged (With Pad):    {:.2?}", start.elapsed());
        println!("{:?}", &wedged);
    }
}