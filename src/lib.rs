use pyo3::prelude::*;
use core::f64::consts::PI;
/// Simulates a plan and returns the distance traveled.
/// Adapted from: https://web.stanford.edu/class/cs168/qwop.py
fn _sim(mut plan: [f64; 40]) -> f64{
    // clip plan to [-1, 1]
    plan.iter_mut().for_each(|x| *x = x.max(-1.0).min(1.0));

    const DT: f64 = 0.1;
    const FRICTION: f64 = 1.0;
    const GRAVITY: f64 = 0.1;
    const MASS: [f64; 6] = [30.0, 10.0, 5.0, 10.0, 5.0, 10.0];
    const EDGEL: [f64; 5] = [0.5, 0.5, 0.5, 0.5, 0.9];
    const EDGESP: [f64; 5] = [160.0, 180.0, 160.0, 180.0, 160.0];
    const EDGEF: [f64; 5] = [8.0, 8.0, 8.0, 8.0, 8.0];
    const ANGLESSP: [f64; 4] = [20.0, 20.0, 10.0, 10.0];
    const ANGLESF: [f64; 4] = [8.0, 8.0, 4.0, 4.0];

    const EDGE: [(usize, usize); 5] = [(0, 1),(1, 2),(0, 3),(3, 4),(0, 5)];
    const ANGLES: [(usize, usize); 4] = [(4, 0),(4, 2),(0, 1),(2, 3)];

    let mut v = [[0.0,0.0,0.0,0.0,0.0,0.0], [0.0,0.0,0.0,0.0,0.0,0.0]];
    let mut p = [[0.0, 0.0, -0.25, 0.25, 0.25, 0.15], [1.0, 0.5, 0.0, 0.5, 0.0, 1.9]];

    let mut spin = 0.0;
    let mut maxspin = 0.0;
    let mut lastang = 0.0;

    for j in 0..20{
        for k in 0..10 {
            let lamb = 0.05 + 0.1*(k as f64);
            let mut t0 = 0.5;
            if j > 0 {
                t0 = plan[2*j - 2];
            }
            t0 *= 1. - lamb;
            t0 += plan[2*j]*lamb;

            let mut t1 = 0.0;
            if j > 0{
                t1 = plan[2*j - 1];
            }
            t1 *= 1. - lamb;
            t1 += plan[2*j + 1]*lamb;

            let mut contact = [false; 6];
            for z in 0..6 {
                if p[1][z] <= 0.0 {
                    contact[z] = true;
                    spin = 0.0;
                    p[1][z] = 0.0;
                }
            }

            let anglesl = [-(2.8+t0), -(2.8-t0), -(1.0-t1)*0.9, -(1.0+t1)*0.9];

            let mut disp = [[0.0; 5]; 2];
            let mut dist = [0.0; 5];
            let mut dispn = [[0.0; 5]; 2];

            for z in 0..5 {
                disp[0][z] = p[0][EDGE[z].1] - p[0][EDGE[z].0];
                disp[1][z] = p[1][EDGE[z].1] - p[1][EDGE[z].0];
                dist[z] = ((disp[0][z]*disp[0][z] + disp[1][z]*disp[1][z]) as f64).sqrt() + 0.01;
                let inv = 1.0/dist[z];
                dispn[0][z] = disp[0][z]*inv;
                dispn[1][z] = disp[1][z]*inv;
            }

            let mut dispv = [[0.0; 5]; 2];
            let mut distv = [0.0; 5];
            for z in 0..5 {
                dispv[0][z] = v[0][EDGE[z].1] - v[0][EDGE[z].0];
                dispv[1][z] = v[1][EDGE[z].1] - v[1][EDGE[z].0];
                distv[z] = 2.0*(disp[0][z]*dispv[0][z] + disp[1][z]*dispv[1][z]);
            }

            let mut forceedge = [[0.0; 5]; 2];
            for z in 0..5 {
                let c = (EDGEL[z] - dist[z])*EDGESP[z] - distv[z]*EDGEF[z];
                forceedge[0][z] = c*dispn[0][z];
                forceedge[1][z] = c*dispn[1][z];
            }

            let mut edgeang = [0.0; 5];
            let mut edgeangv = [0.0; 5];
            for z in 0..5 {
                edgeang[z] = (disp[1][z] as f64).atan2(disp[0][z] as f64);
                edgeangv[z] = (dispv[0][z]*disp[1][z] - dispv[1][z]*disp[0][z])/(dist[z]*dist[z]);
            }

            let mut inc = edgeang[4] - lastang;
            if inc < -PI {
                inc += 2.0*PI;
            } else if inc > PI {
                inc -= 2.0*PI;
            }
            spin += inc;
            let spinc = spin - 0.005*(k + 10*j) as f64;
            if spinc > maxspin {
                maxspin = spinc;
                lastang = edgeang[4];
            }

            let mut angv = [0.0; 4];
            for z in 0..4 {
                angv[z] = edgeangv[ANGLES[z].1] - edgeangv[ANGLES[z].0];
            }

            let mut angf = [0.0; 4];
            for z in 0..4 {
                let mut ang = edgeang[ANGLES[z].1] - edgeang[ANGLES[z].0] - anglesl[z];
                if ang > PI {
                    ang -= 2.0*PI;
                } else if ang < -PI {
                    ang += 2.0*PI;
                }
                let m0 = dist[ANGLES[z].0]/EDGEL[ANGLES[z].0];
                let m1 = dist[ANGLES[z].1]/EDGEL[ANGLES[z].1];
                angf[z] = ang*ANGLESSP[z] - angv[z]*ANGLESF[z]*m0.min(m1);
            }

            let mut edgetorque = [[0.0; 5]; 2];
            for z in 0..5 {
                let inv = 1.0/(dist[z]*dist[z]);
                edgetorque[0][z] = -disp[1][z]*inv;
                edgetorque[1][z] =  disp[0][z]*inv;
            }

            for z in 0..4 {
                let i0 = ANGLES[z].0;
                let i1 = ANGLES[z].1;
                forceedge[0][i0] += angf[z]*edgetorque[0][i0];
                forceedge[1][i0] += angf[z]*edgetorque[1][i0];
                forceedge[0][i1] -= angf[z]*edgetorque[0][i1];
                forceedge[1][i1] -= angf[z]*edgetorque[1][i1];
            }

            let mut f = [[0.0; 6]; 2];
            for z in 0..5 {
                let i0 = EDGE[z].0;
                let i1 = EDGE[z].1;
                f[0][i0] -= forceedge[0][z];
                f[1][i0] -= forceedge[1][z];
                f[0][i1] += forceedge[0][z];
                f[1][i1] += forceedge[1][z];
            }

            for z in 0..6 {
                f[1][z] -= GRAVITY*MASS[z];
                let invm = 1.0/MASS[z];
                v[0][z] += f[0][z]*DT*invm;
                v[1][z] += f[1][z]*DT*invm;

                if contact[z] {
                    let mut fric = 0.0;
                    if v[1][z] < 0.0 {
                        fric = -v[1][z];
                        v[1][z] = 0.0;
                    }

                    let s = v[0][z].signum();
                    if v[0][z]*s < fric*FRICTION {
                        v[0][z] = 0.0;
                    } else {
                        v[0][z] -= fric*FRICTION*s;
                    }
                }
                p[0][z] += v[0][z] * DT;
                p[1][z] += v[1][z] * DT;
            }

            if contact[0] || contact[5] {
                return p[0][5];
            }

        }
    }

    p[0][5]
}

/// Simulates a plan and returns the distance traveled.
#[pyfunction]
fn sim(plan: [f64; 40]) -> PyResult<f64> {
    Ok(_sim(plan))
}

/// A Python module implemented in Rust.
#[pymodule]
fn qwop_fast(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sim, m)?)?;
    Ok(())
}