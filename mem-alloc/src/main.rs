use std::time::Instant;

fn main() {
    let n = 1000000;

    let mut sum = 0.;
    let now = Instant::now();
    for i in 0..n {
        let x = i as f64 + 1.0;
        sum += x;
    }
    let duration = now.elapsed().as_nanos();
    println!("{} in {}ns (Stack)", sum, duration);

    let mut sum = 0.;
    let now = Instant::now();
    let mut x;
    for i in 0..n {
        x = i as f64 + 1.0;
        sum += x;
    }
    let duration = now.elapsed().as_nanos();
    println!("{} in {}ns (Stack Alloc Once)", sum, duration);

    let mut sum = 0.;
    let now = Instant::now();
    for i in 0..n {
        let x = Box::new(i as f64 + 1.0);
        sum += *x;
    }
    let duration = now.elapsed().as_nanos();
    println!("{} in {}ns (Heap)", sum, duration);

    let mut sum = 0.;
    let now = Instant::now();
    let mut x = Box::new(0.0);
    for i in 0..n {
        *x = i as f64 + 1.0;
        sum += *x;
    }
    let duration = now.elapsed().as_nanos();
    println!("{} in {}ns (Heap Alloc Once)", sum, duration);
}
