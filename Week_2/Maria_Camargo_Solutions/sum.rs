/*
Numerically compute the integral of a curve using the Riemann Sum method.
*/
use std::f32::consts::E;

fn main() {
    let DOMAIN = Vec::from([0,10]);
    let INTERVAL = 0.1;

    // calc the number of segments
    let num_seg = (DOMAIN[1] - DOMAIN[0]) as f32 / INTERVAL;

    // calc sum
    let mut total_area = 0.0;
    for seg in (0..num_seg as u32) {
        // calc x coordinates
        let x = seg as f32 * INTERVAL + 0.5 * INTERVAL;
        
        // add area of segment to counter
        total_area += function(x) * INTERVAL;
    }

    // print report
    println!("Integral: {total_area}");
}


// define function
fn function(x: f32) -> f32 {
    let power = -(x.powf(2.0));
    E.powf(power)
}
