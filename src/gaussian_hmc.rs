use rand::prelude::*;
use std::f64::consts::PI;

const NUM_ITER: usize = 5000;
const NUM_TAU: usize = 10;
const DTAU: f64 = 0.1;

/// create gaussian random variable by Box-muller
fn boxmuller() -> (f64, f64) {
    let mut rng = thread_rng();
    let r: f64 = rng.gen();
    let s: f64 = rng.gen();

    let p = (-2.0 * r.ln()).sqrt() * (2.0 * PI * s).sin();
    let q = (-2.0 * r.ln()).sqrt() * (2.0 * PI * s).cos();
    (p, q)
}

/// calculate action S[x]
fn calc_action(x: f64) -> f64 {
    0.5 * x.powi(2)
}

/// calculate hamiltonian H[x, p]
fn calc_hamiltonian(x: f64, p: f64) -> f64 {
    calc_action(x) + 0.5 * p.powi(2)
}

/// calculate dH/dx
fn calc_delh(x: f64) -> f64 {
    x
}

/// time evolution by molecular orbital method
fn molecular_dynamics(x: f64) -> (f64, f64, f64) {
    let (mut p, _q) = boxmuller();
    let ham_init = calc_hamiltonian(x, p);

    // first step
    let mut inner_x = x + p * 0.5 * DTAU;

    // second, ... Ntau-th steps
    for _i in 1..NUM_TAU {
        let delh = calc_delh(inner_x);
        p += -delh * DTAU;
        inner_x += p * DTAU;
    }

    // last step
    let delh = calc_delh(inner_x);
    p += -delh * DTAU;
    inner_x += p * 0.5 * DTAU;

    let ham_fin = calc_hamiltonian(inner_x, p);

    (inner_x, ham_init, ham_fin)
}

fn main() {
    let mut x = 0.0;
    let mut n_accept = 0;
    let mut sum_xx = 0.0;
    let mut rng = thread_rng();

    for i in 1..NUM_ITER {
        let backup_x = x;
        // leap frog
        let (dyna_x, ham_init, ham_fin) = molecular_dynamics(x);

        let metropolis: f64 = rng.gen();
        if (ham_init - ham_fin).exp() > metropolis {
            x = dyna_x;
            n_accept += 1;
        } else {
            x = backup_x;
        }

        // data output
        sum_xx += x.powi(2);
        println!("{} {} {}", x, sum_xx / i as f64, n_accept as f64 / i as f64);
    }
}
