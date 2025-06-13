
use std::f64::consts;

fn eval_point(point : &f64, step : &f64) -> f64 {

    let height = consts::E.powf(-(point.powf(2.0)));
    height * step



}

fn main() {
    let step : f64 = 0.00001;
    let mut place : f64 = -100.0;
    let stop : f64 = 100.0;

    let mut vec_of_areas : Vec<f64> = vec![0.0; ((stop - place) * (1.0/ step)) as usize];
    
    for i in 0..vec_of_areas.len(){
        vec_of_areas[i] = eval_point(&place, &step);
        place += step
    }

    let area : f64 = vec_of_areas.iter().sum();
    let error : f64 = consts::PI.powf(0.5) - area;
    println!("the area under the curve was {}", area);
    println!("the error is {}", error.abs());

}
