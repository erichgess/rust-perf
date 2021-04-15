use std::time::Instant;

fn main() {
    // Allocate a large number of integers on the heap
    // Then every 1000 entries drop the integer
    /*let mut v0 = vec![];
    for _ in 0..5 {
        v0.push(Box::new(0.))
    }
    let mut v = vec![];
    for _ in 0..10000 {
        v.push(Box::new(0.))
    }
    let mut v1 = vec![];
    for _ in 0..5 {
        v1.push(Box::new(0.))
    }

    drop(v0);
    drop(v1);*/

    let a = Box::new(1.);
    let b = Box::new(2.);
    let c = Box::new(3.);
    let d = Box::new(4.);
    let e = Box::new(5.);

    let f = Box::new(6.);
    let g = Box::new(7.);
    let h = Box::new(8.);
    let i = Box::new(9.);
    let j = Box::new(10.);

    let mut sum = 0.;
    let now = Instant::now();
    for _ in 0..10000000 {
        sum += *a + *b + *c + *d + *e + *f + *g + *h + *i + *j;
    }
    let duration = now.elapsed().as_nanos();
    println!("{} in {}ns", sum, duration);
    println!("{:p} -> {:p}", a, b);
}
