use std::thread;
use std::time::Instant;

fn main() {
    let loops = 100_000_000;

    for threads in 1..=8 {
        n_threads(threads, loops / threads);
    }
}

fn n_threads(num_threads: usize, loops: usize) {
    let sw = Instant::now();

    let mut threads = Vec::new();
    for idx in 0..num_threads {
        let t = thread::spawn(move || {
            let v = work(loops);
            (idx, v)
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
