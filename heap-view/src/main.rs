use std::time::Instant;

fn main() {
    linear_sum();
    bounce();
    skipper_sum();
    linear_sum();
}

fn skipper_sum() -> f64 {
    let n = 100_000;
    let a = vec![1.; n];
    let mut sum = 0.;

    let time = Instant::now();
    let n = n / 10;
    for i in 0..10 {
        for j in 0..n {
            sum += a[j * 10 + i];
        }
    }
    let d = time.elapsed().as_nanos();
    println!("{} in {}ns", sum, d); /* */

    sum
}

fn linear_sum() -> f64 {
    let a = vec![1.; 100_000];
    let mut sum = 0.;

    let time = Instant::now();
    for i in 0..50_000 {
        sum += a[i];
        sum += a[i + 1];
    }
    let d = time.elapsed().as_nanos();
    println!("{} in {}ns", sum, d); /* */

    sum
}

fn bounce() -> f64 {
    let a = vec![1.; 50_000];
    let b = vec![1.; 50_000];
    let mut sum = 0.;

    let time = Instant::now();
    for i in 0..100_000 / 2 {
        sum += a[i];
        sum += b[i];
    }
    let d = time.elapsed().as_nanos();
    println!("{} in {}ns", sum, d); /* */

    sum
}
