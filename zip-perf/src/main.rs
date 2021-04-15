use std::{collections::HashMap, time::Instant};

fn main() {
    // array sizes
    let sizes = [100, 1000, 10_000, 100_000, 1_000_000];
    let last = sizes.last().unwrap();
    // Prime the heap so performance isn't impacted by syscalls
    prime_heap(*last);

    let mut zip_1_dur: HashMap<usize, Vec<u128>> = HashMap::new();
    let mut zip_2_dur: HashMap<usize, Vec<u128>> = HashMap::new();

    // Run zip_1 and zip_2 10 times for each array size
    let runs = 20;

    for i in &[100, 1000, 10_000, 100_000, 1_000_000] {
        zip_1_dur.insert(*i, vec![]);
        zip_2_dur.insert(*i, vec![]);
        for _ in 0..runs {
            let (_, d1) = zip_1(*i);
            let (_, d2) = zip_2(*i);

            zip_1_dur.get_mut(i).map(|v| v.push(d1));
            zip_2_dur.get_mut(i).map(|v| v.push(d2));
        }
    }

    // Print average times for each step
    for i in &[100, 1000, 10_000, 100_000] {
        // Compute the average duration across the `runs`
        let sum_1: u128 = zip_1_dur.get(i).map(|v| v.iter().sum()).unwrap();
        let sum_2: u128 = zip_2_dur.get(i).map(|v| v.iter().sum()).unwrap();

        println!("\n---{}---", i);
        println!("zip_1: {} ns", (sum_1 as f64) / (runs as f64));
        println!("zip_2: {} ns", (sum_2 as f64) / (runs as f64));
        println!("---");
    }
}

fn prime_heap(sz: usize) -> usize {
    let v = vec![0.; sz];
    v.len()
}

fn zip_1(n: usize) -> (f64, u128) {
    let a = vec![1.; n];
    let b = vec![1.; n];
    let c = vec![1.; n];
    let d = vec![1.; n];

    let z = a.iter().zip(b.iter().zip(c.iter().zip(d.iter())));
    let timer = Instant::now();
    let mut sum = 0.;
    for (a1, (b1, (c1, d1))) in z {
        sum += a1 + b1 + c1 + d1;
    }
    let dur = timer.elapsed().as_nanos();

    (sum, dur)
}

fn zip_2(n: usize) -> (f64, u128) {
    let a = vec![1.; n];
    let b = vec![1.; n];
    let c = vec![1.; n];
    let d = vec![1.; n];

    let z = a.iter().zip(b.iter()).zip(c.iter()).zip(d.iter());
    let timer = Instant::now();
    let mut sum = 0.;
    for (((a1, b1), c1), d1) in z {
        sum += a1 + b1 + c1 + d1;
    }
    let dur = timer.elapsed().as_nanos();

    (sum, dur)
}
