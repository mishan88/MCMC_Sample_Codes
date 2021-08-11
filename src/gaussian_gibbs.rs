use rand::prelude::*;
use std::f64::consts::PI;

const NUM_ITER: usize = 10000;

/// create gaussian random variable by Box-muller
fn boxmuller() -> (f64, f64) {
    let mut rng = thread_rng();
    let r: f64 = rng.gen();
    let s: f64 = rng.gen();

    let p = (-2.0 * r.ln()).sqrt() * (2.0 * PI * s).sin();
    let q = (-2.0 * r.ln()).sqrt() * (2.0 * PI * s).cos();
    (p, q)
}

fn main() {
    let a: Vec<Vec<f64>> = vec![vec![1., 1., 1.], vec![1., 2., 1.], vec![1., 1., 2.]];

    let mut y = 0.0;
    let mut z = 0.0;

    for _i in 0..NUM_ITER {

        // update x
        let sigma_x = 1.0 / a[0][0].sqrt();
        let mu_x = -a[0][1] / a[0][0] * y - a[0][2] / a[0][0] * z;
        let (r1_x, _r2_x) = boxmuller();
        let x = sigma_x * r1_x + mu_x;

        // update y
        let sigma_y = 1.0 / a[1][1].sqrt();
        let mu_y = -a[1][0] / a[1][1] * x - a[1][2] / a[1][1] * z;
        let (r1_y, _r2_y) = boxmuller();
        y = sigma_y * r1_y + mu_y;

        // update z
        let sigma_z = 1.0 / a[2][2].sqrt();
        let mu_z = -a[2][0] / a[2][2] * x - a[2][1] / a[2][2] * y;
        let (r1_z, _r2_z) = boxmuller();
        z = sigma_z * r1_z + mu_z;

        println!("{}, {}, {}", x, y, z);
    }
}