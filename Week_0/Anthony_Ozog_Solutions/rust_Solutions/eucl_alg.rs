use std::io;
fn main() {

    let mut line_1 = String::new();
    let mut line_2 = String::new();

    io::stdin()
        .read_line(&mut line_1)
        .expect("Failed to read line");
    
    io::stdin()
        .read_line(&mut line_2)
        .expect("Failed to read line");

    let num_1 : usize = line_1.trim().parse::<usize>().expect("needs to be a number");
    let num_2 : usize = line_2.trim().parse::<usize>().expect("needs to be a number");

    if num_1 > num_2{
    println!("{}", gcd(num_1,num_2))
    }else{
    println!("{}", gcd(num_2,num_1))
    }
}



fn gcd(a : usize, b : usize) -> usize {
    if a % b == 0 {
        return b
    }
    return gcd(b , a % b)
}
