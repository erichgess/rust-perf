use std::thread;
use std::time::Instant;

fn main() {
    let loops = 100_000_000;

    for threads in 1..=8 {
        // As threads are added to the test, evenly split the total number of iterations
        // across all threads, so that 1 thread test can be compared to 4 thread test.
        // For `threads` that are not divisors of `loops` some threads may have one more
        // iteration than the others but that will be 1 out of 10,000,000 and should have
        // negligible effect on the run time.
        n_threads(threads, loops / threads);
    }
}

/// Have `num_threads` threads each run a function that will
/// iterate a computation `loops` times.
fn n_threads(num_threads: usize, loops: usize) {
    let sw = Instant::now();

    let mut threads = Vec::new();
    for _ in 0..num_threads {
        let t = thread::spawn(move || {
            let v = work(loops);
            v
        });
        threads.push(t);
    }

    for t in threads.into_iter() {
        t.join().unwrap();
    }

    let time = sw.elapsed();
    println!("{}, {}", num_threads, time.as_millis());
}

fn work(loops: usize) -> f64 {
    let mut x = 0.5;

    for i in 0..loops {
        x += (i as f64 / 10000.).sin();
    }

    x
}
