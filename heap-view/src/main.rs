use std::time::Instant;

fn main() {
    linear_sum();

    bounce();

    linear_sum();
}

fn linear_sum() -> f64 {
    let a = Box::new([1.; 100_000]);
    let mut sum = 0.;

    let time = Instant::now();
    for i in 0..100_000 {
        sum += a[i];
    }
    let d = time.elapsed().as_nanos();
    println!("{} in {}ns", sum, d); /* */

    sum
}

fn bounce() -> f64 {
    let a = Box::new([1.; 100_000]);
    let mut sum = 0.;

    let time = Instant::now();
    for i in 0..100_000 / 2 {
        sum += a[i];
        let j = 100_000 - i - 1;
        sum += a[j];
    }
    let d = time.elapsed().as_nanos();
    println!("{} in {}ns", sum, d); /* */

    sum
}
