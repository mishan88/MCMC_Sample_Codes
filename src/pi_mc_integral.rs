use rand::prelude::*;

const NUM_ITER: usize = 1000;

fn main() {
    let mut sum_y = 0.0;
    let mut rng = thread_rng();

    for i in 1..NUM_ITER {
        let x: f64 = rng.gen();
        let y = (1.0 - x * x).sqrt();
        sum_y += y;
        println!("{}, {}", i, sum_y / i as f64);
    }
}
