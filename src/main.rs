const N_P: u16 = 21;
const N_X: u16 = 40;
const N_Y: u16 = 20;


fn main() {
    let mut state: Vec<bool> = Vec::with_capacity(N_X * N_Y);
    let mut target: Vec<f64> = Vec::with_capacity(N_P);
    let error = calculate_mse(state, target);
    println!("{error}")
    println!("Hello, world!");
}


fn calculate_mse(state: Vec<bool>, target: Vec<f64>) -> f64 {
let mut total_error: f64 = 0.0;
for t_val in target{
for (idx, magnet) in state.enumerate(){
total_error += 1.0;
}
}
total_error
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
