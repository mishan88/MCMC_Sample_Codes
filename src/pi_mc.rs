use rand::prelude::*;

const NUM_ITER: usize = 10000;

fn main() {
    let mut n_in = 0;
    let mut rng = thread_rng();
    for i in 1..NUM_ITER {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();

        if x * x + y * y < 1.0 {
            n_in += 1;
        }
        println!("{}, {} ", i, (n_in as f64 / i as f64));
    }
}
