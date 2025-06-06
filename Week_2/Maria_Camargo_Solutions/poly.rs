use std::{char, io, ops::Index};

/*
Given a Polynomial, compute its derivative and integral.
For example:

    2 x 2 + 4 x + 5
    Derivative: 4 x + 4
    Integral: 2 3 x 3 + 2 x 2 + 5 x + C

If you want, you can make these visually appealing and symbolic, but for simplicity, you can also represent input and output as vectors.
For example:

    2 x 2 + 4 x + 5
    [2,4,5]
*/
fn main() {
    println!("Input: [a,b,c], where ax^2+bx+c");
    println!("Polynomial: ");

    // Get input
    let mut poly = String::new();

    io::stdin()
        .read_line(&mut poly)
        .expect("Failed to read line");

    // store the string as a vector
    // [1, 2, 3]
    // trim string
    poly = poly.replace("[", "");
    poly = poly.replace("]", "");

    // split string and convert
    let temp: Vec<String> = poly
        .split(',')
        .map(|s| s.trim().parse::<String>().unwrap())
        .collect();
    
    // parse each element into an integer
    let coeficients: Vec<f32> = temp
        .iter()
        .map(|s| s.parse::<f32>().unwrap())
        .collect();

    // calculate derivative
    let mut max_exponent_d:i32 = coeficients.len() as i32 - 1;
    let mut max_exponent_i:i32 = coeficients.len() as i32;
    let mut derivative: Vec<f32> = Vec::new();
    let mut integral: Vec<f32> = Vec::new();
    for x in coeficients {
        // calc derivative
        if max_exponent_d > 0 {
            derivative.push(max_exponent_d as f32 * x as f32);
            max_exponent_d -= 1;
        }
        // calc integral
        integral.push(x as f32 / max_exponent_i as f32);
        max_exponent_i -= 1;
    }
    // append 0 as a constant for integral
    integral.push(0.0);

    // print report
    print!("Derivative: [");
    let l = derivative.len();
    let mut  counter = 1;
    for element in derivative {
        if counter == l {
            print!("{element}");
        }
        else {
            print!("{element},");
        }
        counter += 1;
    }
    println!("]");

    print!("Integral: [");
    let l = integral.len();
    let mut  counter = 1;
    for element in integral {
        if counter == l {
            print!("{element}");
        }
        else {
            print!("{element},");
        }
        counter += 1;
    }
    println!("]");
}

