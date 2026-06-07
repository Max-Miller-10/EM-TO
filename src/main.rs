const N_P: usize = 21;
const N_X: usize = 40;
const N_Y: usize = 20;
const WIDTH: f64 = 0.010;
const K_MAX: usize = 1000;
struct RegParameters {
    c_w: f64,
    c_h: f64,
    c_b: f64,
    c_s: f64,
}

fn main() {
    let mut state: Vec<bool> = Vec::with_capacity(N_X * N_Y);
    let mut target: Vec<f64> = Vec::with_capacity(N_P);
    let rps = RegParameters {
        c_w: 0.1,
        c_h: 0.1,
        c_b: 0.1,
        c_s: 0.1,
    };

    for _ in 0..(N_X * N_Y) {
        state.push(true);
    }

    for _ in 0..N_P {
        target.push(1.0);
    }

    let error = calculate_error(&state, &target, rps);
    println!("{error}");
}

// fn simulateAnnealing() {
//     let mut temp = 1.0;
//     for k in 0..K_MAX {}
// }

fn calculate_error(state: &[bool], target: &[f64], rps: RegParameters) -> f64 {
    calculate_mse(state, target) + calculate_regularisation_penalty(state, rps)
}

fn calculate_mse(state: &[bool], target: &[f64]) -> f64 {
    let mut total_error: f64 = 0.0;
    for (jdx, &t_val) in target.iter().enumerate() {
        let mut node_error = -t_val;
        for (idx, &active) in state.iter().enumerate() {
            if active {
                node_error += calculate_by_2d_magnet(
                    WIDTH,
                    (
                        (idx % N_X) as f64 * WIDTH + WIDTH / 2.0,
                        (idx / N_X) as f64 * WIDTH + WIDTH / 2.0,
                    ),
                    (
                        (WIDTH * (N_X) as f64) * (jdx as f64 / (N_P - 1) as f64),
                        0.010 + WIDTH / 2.0,
                    ),
                    1.0,
                )
            }
        }
        total_error += node_error * node_error;
    }
    total_error
}

fn calculate_regularisation_penalty(state: &[bool], rps: RegParameters) -> f64 {
    let mut penalty = 0.0;

    let mut w_penalty = 0.0;
    for jdx in 0..N_Y {
        for idx in 1..(N_X - 1) {
            w_penalty += ((state[idx - 1 + jdx * N_X] as i8 - state[idx + N_X * jdx] as i8)
                * (state[idx + 1 + jdx * N_X] as i8 - state[idx + jdx * N_X] as i8))
                as f64;
        }
    }
    penalty += w_penalty * rps.c_w;

    let mut h_penalty = 0.0;
    for jdx in 1..(N_Y - 1) {
        for idx in 0..(N_X) {
            h_penalty += ((state[idx + (jdx - 1) * N_X] as i8 - state[idx + N_X * jdx] as i8)
                * (state[idx + (1 + jdx) * N_X] as i8 - state[idx + jdx * N_X] as i8))
                as f64;
        }
    }
    penalty += h_penalty * rps.c_h;

    // let mut b_penalty = 0.0;
    // for jdx in 0..N_Y {
    //     for idx in 1..(N_X - 1) {
    //         w_penalty += ((state[idx - 1 + jdx * N_X] as i8 - state[idx + N_X * jdx] as i8)
    //             * (state[idx + 1 + jdx * N_X] as i8 - state[idx + jdx * N_X] as i8))
    //             as f64;
    //     }
    // }
    // penalty += w_penalty * rps.c_w;

    // let mut w_penalty = 0.0;
    // for jdx in 0..N_Y {
    //     for idx in 1..(N_X - 1) {
    //         w_penalty += ((state[idx - 1 + jdx * N_X] as i8 - state[idx + N_X * jdx] as i8)
    //             * (state[idx + 1 + jdx * N_X] as i8 - state[idx + jdx * N_X] as i8))
    //             as f64;
    //     }
    // }
    penalty += w_penalty * rps.c_w;

    penalty
}

pub fn calculate_by_2d_magnet(
    width: f64,
    magnet_pos: (f64, f64),
    eval_pos: (f64, f64),
    b_r: f64,
) -> f64 {
    let (x_c, y_c) = magnet_pos;
    let (x, y) = eval_pos;

    // Calculate the physical boundaries of the square magnet
    let x_l = x_c - width / 2.0;
    let x_r = x_c + width / 2.0;
    let y_bottom = y_c - width / 2.0;
    let y_top = y_c + width / 2.0;

    // Contribution from the Left surface current sheet (current pointing out-of-page, +z)
    let term_l_top = ((y_top - y) / (x - x_l)).atan();
    let term_l_bottom = ((y_bottom - y) / (x - x_l)).atan();
    let b_y_left = term_l_top - term_l_bottom;

    // Contribution from the Right surface current sheet (current pointing into-page, -z)
    let term_r_top = ((y_top - y) / (x - x_r)).atan();
    let term_r_bottom = ((y_bottom - y) / (x - x_r)).atan();
    let b_y_right = term_r_top - term_r_bottom;

    // Total By field scaled by the scale factor: Br / (2 * PI)
    // The right side is subtracted due to the inverted bound current direction.
    let scale_factor = b_r / (2.0 * std::f64::consts::PI);

    scale_factor * (b_y_left - b_y_right)
}
