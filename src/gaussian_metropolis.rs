use rand::prelude::*;

const NUM_ITER: usize = 100000;
const STEP_SIZE: f64 = 0.5;

fn main() {
    let mut x = 0.0;
    let mut num_accept = 0;
    let mut rng = thread_rng();

    for i in 1..NUM_ITER {
        let backup_x = x;
        let action_init = 0.5 * x * x;

        let dx: f64 = (rng.gen_range(-0.5..0.5)) * STEP_SIZE * 2.0;
        x = x + dx;
        let action_fin = 0.5 * x * x;

        // metropolis test
        let metropolis: f64 = rng.gen();
        if ((action_init - action_fin) as f64).exp() > metropolis {
            num_accept += 1;
        } else {
            x = backup_x;
        }
        println!("{} {}", x, num_accept as f64 / i as f64);
    }
}
