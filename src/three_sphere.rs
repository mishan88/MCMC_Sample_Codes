use rand::prelude::*;
use std::f64::consts;

const NUM_ITER: usize = 100000;

fn main() {
    let mut rng = thread_rng();
    let mut sum_z = 0.0;

    let mut n_in = 0;

    for i in 0..NUM_ITER {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();

        if x * x + y * y < 1.0 {
            n_in += 1;
            let z = (1.0 - x * x - y * y).sqrt();
            sum_z += z;
        }
        println!("{}, {}", i, sum_z as f64 / n_in as f64 * 2.0 * consts::PI);
    }
}
