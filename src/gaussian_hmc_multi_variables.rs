use rand::prelude::*;
use std::f64::consts::PI;

const NUM_ITER: usize = 10000;
const NUM_TAU: usize = 20;
const DTAU: f64 = 0.1;
const NDIM: usize = 3;

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
fn calc_action(x: Vec<f64>, a: Vec<Vec<f64>>) -> f64 {
    let mut action = 0.0;

    for i in 0..NDIM {
        for j in 0..NDIM {
            action += x[i] * a[i][j] * x[j];
        }
        action += 0.5 * x[i] * a[i][i] * x[i];
    }
    action
}

/// calculate hamiltonian H[x, p]
fn calc_hamiltonian(x: Vec<f64>, p: Vec<f64>, a: Vec<Vec<f64>>) -> f64 {
    let mut ham = calc_action(x, a);
    for i in 0..NDIM {
        ham += 0.5 * p[i].powi(2);
    }

    ham
}

/// calculate dH/dx
fn calc_delh(x: Vec<f64>, a: Vec<Vec<f64>>) -> Vec<f64> {
    let mut delh = vec![0.0, 0.0, 0.0];
    for i in 0..NDIM {
        for j in 0..NDIM {
            delh[i] += a[i][j] * x[j]
        }
    }

    delh
}

/// time evolution by molecular orbital method
fn molecular_dynamics(x: Vec<f64>, a: Vec<Vec<f64>>) -> (Vec<f64>, Vec<Vec<f64>>, f64, f64) {
    let mut p = Vec::with_capacity(NDIM);

    let mut inner_x = vec![0.0, 0.0, 0.0];

    for _i in 0..NDIM {
        let (r1, _r2) = boxmuller();
        p.push(r1);
    }

    let ham_init = calc_hamiltonian(x.clone(), p.clone(), a.clone());

    for i in 0..NDIM {
        inner_x[i] = x[i] + p[i] * 0.5 * DTAU;
    }

    // second, ... Ntau-th steps
    for _i in 1..NUM_TAU {
        let delh = calc_delh(inner_x.clone(), a.clone());
        for i in 0..NDIM {
            p[i] -= delh[i] * DTAU;
        }
        for i in 0..NDIM {
            inner_x[i] += p[i] * DTAU;
        }
    }

    // last step
    let delh = calc_delh(inner_x.clone(), a.clone());
    for i in 0..NDIM {
        p[i] -= delh[i] * DTAU;
    }
    for i in 0..NDIM {
        inner_x[i] += p[i] * 0.5 * DTAU;
    }

    let ham_fin = calc_hamiltonian(inner_x.clone(), p, a.clone());

    (inner_x, a, ham_init, ham_fin)
}

fn main() {
    let mut x = vec![0., 0., 0.];
    let a = vec![vec![1., 1., 1.], vec![1., 2., 1.], vec![1., 1., 2.]];

    let mut rng = thread_rng();
    let mut n_accept = 0;

    for i in 1..NUM_ITER {
        let backup_x = x.clone();
        let (dyna_x, _a, ham_init, ham_fin) = molecular_dynamics(x.clone(), a.clone());

        let metropolis: f64 = rng.gen();
        if (ham_init - ham_fin).exp() > metropolis {
            x = dyna_x;
            n_accept += 1;
        } else {
            x = backup_x;
        }
        println!("{:?}, {}", x, n_accept as f64 / i as f64);
    }
}
