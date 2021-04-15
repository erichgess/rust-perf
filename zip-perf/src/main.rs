use std::{collections::HashMap, time::Instant};

use rand::random;

fn main() {
    // array sizes
    let sizes = [100, 1000, 10_000, 100_000, 1_000_000];
    let last = sizes.last().unwrap();
    // Prime the heap so performance isn't impacted by syscalls
    prime_heap(*last);

    let mut zip_1_dur: HashMap<usize, Vec<(f64, u128)>> = HashMap::new();
    let mut zip_2_dur: HashMap<usize, Vec<(f64, u128)>> = HashMap::new();

    // Run zip_1 and zip_2 10 times for each array size
    let runs = 20;

    for i in &sizes {
        zip_1_dur.insert(*i, vec![]);
        zip_2_dur.insert(*i, vec![]);
        for _ in 0..runs {
            let (s1, d1) = zip_1(*i);
            let (s2, d2) = zip_2(*i);

            zip_1_dur.get_mut(i).map(|v| v.push((s1, d1)));
            zip_2_dur.get_mut(i).map(|v| v.push((s2, d2)));
        }
    }

    // Print average times for each step
    for i in &sizes {
        // Compute the average duration across the `runs`
        let sum_1: u128 = zip_1_dur
            .get(i)
            .map(|v| v.iter().map(|(_, d)| d).sum())
            .unwrap();
        let sum_2: u128 = zip_2_dur
            .get(i)
            .map(|v| v.iter().map(|(_, d)| d).sum())
            .unwrap();

        println!("\n---{}---", i);
        println!("zip_1: {} ns", (sum_1 as f64) / (runs as f64));
        println!("zip_2: {} ns", (sum_2 as f64) / (runs as f64));
        println!("---");
    }
}

fn prime_heap(sz: usize) -> usize {
    let v1 = vec![0.; sz];
    let v2 = vec![0.; sz];
    let v3 = vec![0.; sz];
    let v4 = vec![0.; sz];
    let v5 = vec![0.; sz];
    let v6 = vec![0.; sz];
    v1.len() + v2.len() + v3.len() + v4.len() + v5.len() + v6.len()
}

fn fill_random(v: &mut Vec<f64>) {
    let l = v.len();
    for i in 0..l {
        v[i] = random();
    }
}

fn zip_1(n: usize) -> (f64, u128) {
    let mut a = vec![1.; n];
    let mut b = vec![1.; n];
    let mut c = vec![1.; n];
    let mut d = vec![1.; n];

    fill_random(&mut a);
    fill_random(&mut b);
    fill_random(&mut c);
    fill_random(&mut d);

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
    let mut a = vec![1.; n];
    let mut b = vec![1.; n];
    let mut c = vec![1.; n];
    let mut d = vec![1.; n];

    fill_random(&mut a);
    fill_random(&mut b);
    fill_random(&mut c);
    fill_random(&mut d);

    let z = a.iter().zip(b.iter()).zip(c.iter()).zip(d.iter());
    let timer = Instant::now();
    let mut sum = 0.;
    for (((a1, b1), c1), d1) in z {
        sum += a1 + b1 + c1 + d1;
    }
    let dur = timer.elapsed().as_nanos();

    (sum, dur)
}
