use std::io;

fn poly_printer<T : std::fmt::Display> (poly : &Vec<T>) {

    let lenght = poly.len(); 
    let mut print_me = format!("{}x^{}", poly[0], lenght - 1 );

    for i in 1..lenght{
        let cur_degree :usize = lenght - 1 - i;
        if cur_degree != 0 {
            print_me = format!("{} + {}x^{}", print_me, poly[i], cur_degree);
        } else {
            print_me = format!("{} + {}", print_me, poly[i]);
        }
}

println!("{}",print_me);
}

fn poly_der (poly : &Vec<usize>) -> Vec<usize>{
    let mut der_poly = vec![0; poly.len()];


    for i in 0..poly.len(){
        der_poly[i] = (poly.len() - 1 - i) * poly[i]
    }
    return (&der_poly[0..poly.len() - 1]).to_vec()
}

fn poly_int(poly : &Vec<usize>) -> Vec<f64>{
    let mut int_poly : Vec<f64> = vec![1.0; poly.len() + 1];
       
    for i in 0..poly.len(){
        int_poly[i] = (poly[i] as f64/(poly.len() - i) as f64) as f64;
    }

    return int_poly



}







fn main() {
    let mut first_line = String::new();
    println!("what degree is your polynomial");

    io::stdin()
        .read_line(&mut first_line)
        .expect("Failed to read line");



    let n: usize = first_line.trim().parse::<usize>().expect("Please type a number!") + 1;

    
    let mut org_poly : Vec<usize> = vec![0; n];
    for i in 0..n {
        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        org_poly[i] = line.trim().parse().expect("Please type a number!");
    }
    
    let der_poly = poly_der(&org_poly);
    let int_poly = poly_int(&org_poly);
    
    println!("The given polynomial was -");
    poly_printer(&org_poly);
    println!("Its derivative is - ");
    poly_printer(&der_poly);
    println!("Its intergral is - ");
    poly_printer(&int_poly);


}




