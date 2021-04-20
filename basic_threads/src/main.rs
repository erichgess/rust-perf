extern crate core_affinity;

use std::thread;
use std::time::Instant;

#[derive(Debug)]
struct Test {
    id: usize,
    time_ms: u128,
    thread_ms: Vec<(usize, f64, u128)>,
}

#[derive(Debug)]
struct TestSet {
    threads: usize,
    loops: usize,
    time: u128,
    tests: Vec<Test>,
}

fn main() {
    print_test_set_header();
    let tests = [5_000_000, 50_000_000];

    for loops in &tests {
        for threads in 1..=8 {
            let ts = n_threads(threads, *loops, 2);
            print_test_set(*loops, "max", &ts);
        }
    }

    for loops in &tests {
        for threads in 1..=8 {
            let ts = n_threads(threads, *loops / threads, 2);
            print_test_set(*loops, "split", &ts);
        }
    }
}

fn print_test_set_header() {
    println!("Test,Type,Th,Loops,Set Sz,Set Time,Avg,Min,Max");
}

fn print_test_set(test: usize, ty: &str, tests: &TestSet) {
    let num_tests = tests.tests.len();
    let thread_times: Vec<u128> = tests
        .tests
        .iter()
        .flat_map(|t| t.thread_ms.iter())
        .map(|(_, _, dt)| *dt)
        .collect();
    let avg = thread_times.iter().sum::<u128>() as f64 / thread_times.len() as f64;
    let max = thread_times.iter().max().unwrap();
    let min = thread_times.iter().min().unwrap();

    println!(
        "{},{},{},{},{},{},{:.2},{},{}",
        test, ty, tests.threads, tests.loops, num_tests, tests.time, avg, min, max
    );
}

fn n_threads(num_threads: usize, loops: usize, tests: usize) -> TestSet {
    // Get CPU information
    let core_ids = core_affinity::get_core_ids().unwrap();
    if num_threads > core_ids.len() {
        panic!("Requested more threads than there are CPU cores");
    }

    let mut thread_stats = vec![];
    let set_timer = Instant::now();
    for _ in 0..tests {
        let sw = Instant::now();
        let mut threads = Vec::new();
        for idx in 0..num_threads {
            let core = core_ids[idx];

            let t = thread::spawn(move || {
                //core_affinity::set_for_current(core);

                let (v, t) = work(loops);
                (idx, v, t)
            });
            threads.push(t);
        }

        let mut results = Vec::new();
        for t in threads.into_iter() {
            match t.join() {
                Ok((idx, v, t)) => {
                    results.push((idx, v, t));
                }
                Err(e) => panic!("{:?}", e),
            }
        }
        let time = sw.elapsed();
        let ts = results.iter().map(|(idx, v, t)| (*idx, *v, *t)).collect();
        thread_stats.push(Test {
            id: 0,
            time_ms: time.as_millis(),
            thread_ms: ts,
        });
    }

    TestSet {
        loops,
        threads: num_threads,
        tests: thread_stats,
        time: set_timer.elapsed().as_millis(),
    }
}

fn work(loops: usize) -> (f64, u128) {
    let mut x = 0.5;
    let sw = Instant::now();
    for i in 0..loops {
        x += (i as f64 / 10000.).sin();
    }
    let dur = sw.elapsed().as_millis();

    (x, dur)
}
