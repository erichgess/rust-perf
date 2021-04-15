use std::time::Instant;
use std::mem;

fn main() {
    for i in &[1000 as usize, 10_000, 100_000, 1_000_000, 10_000_000] {
        let total_bytes = i * mem::size_of::<f64>();
        let kb = total_bytes / 1024;
        println!("{} KiB of Data", kb);
        bounce(*i);
        skipper_sum(*i);
        linear_sum(*i);

        println!("-----");
    }
}

/// This sums the numbers in an array of 100,0000 64bit floats
/// Rather than sum the contiguous line of numbers from 0 to
/// 100,000; it does jumps of 10.  This will skip over 10*8=80
/// bytes of memory for each addition. Cache lines are generally
/// 64 bytes in size, so this guarantees that each addition will
/// hit a different cache line.
fn skipper_sum(n: usize) -> f64 {
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
    println!("skiper: {} in {} ns", sum, d); /* */

    sum
}

/// Sum 100,000 numbers all in a straight line.
/// This will add all the numbers in a cacheline
/// before moving to the next cacheline.
fn linear_sum(n: usize) -> f64 {
    let a = vec![1.; n];
    let mut sum = 0.;

    let time = Instant::now();
    for i in 0..n {
        sum += a[i];
    }
    let d = time.elapsed().as_nanos();
    println!("linear: {} in {} ns", sum, d); /* */

    sum
}

/// This adds 100,0000 numbers that are stored
/// in two different regions of memory.  It starts
/// at the beginning of each array and moves through
/// them at the same time.
///
/// I originally thought this would cause a lot of cache
/// line misses, but I think because each array is
/// traversed contiguously that the L1 cache is more than
/// big enough to keep teh current cache line for both
/// arrays.
fn bounce(n: usize) -> f64 {
    let n2 = n / 2;
    let a = vec![1.; n2];
    let b = vec![1.; n2];
    let mut sum = 0.;

    let time = Instant::now();
    for i in 0..n2 {
        sum += a[i];
        sum += b[i];
    }
    let d = time.elapsed().as_nanos();
    println!("bounce: {} in {} ns", sum, d); /* */

    sum
}
