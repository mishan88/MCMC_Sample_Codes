use rand::prelude::*;

const NUM_ITER: usize = 10000;
const STEP_SIZE_X: f64 = 0.5;
const STEP_SIZE_Y: f64 = 0.5;

fn main() {
    let mut rng = thread_rng();

    let mut x = 0.0;
    let mut y = 0.0;
    let mut n_accept = 0;

    for i in 0..NUM_ITER {
        let backup_x = x;
        let backup_y = y;
        let action_init = 0.5 * (x * x + y * y + x * y);

        let dx: f64 = rng.gen_range(-0.5..0.5) * STEP_SIZE_X * 2.0;
        let dy: f64 = rng.gen_range(-0.5..0.5) * STEP_SIZE_Y * 2.0;

        x += dx;
        y += dy;
        let action_fin = 0.5 * (x * x + y * y + x * y);

        let metropolis = rng.gen();
        if (action_init - action_fin).exp() > metropolis {
            n_accept += 1;
        } else {
            x = backup_x;
            y = backup_y;
        }
        if i % 10 == 0 {
            println!("{}, {}, {}", x, y, n_accept as f64 / i as f64);
        }
    }
}
