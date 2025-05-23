use std::io;

fn main() {
    let mut value = String::new();

    // get user input
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    let value: u32 = value.trim().parse().expect("Please type a number!");

    // loop [1..N] checking for primes
    for i in 1..value {
        // if i is prime, print it in its own line
        if primality(i as f32) == true {
            println!("{i}");
        }
    }

}


/*
Primality rest algorithm
for i : [2,√n]
   if i divides N
     return "Composite"
     
return "Prime" 
*/
fn primality(value: f32) -> bool {
    let mut value_sqrt = value.sqrt().ceil() as i32 + 1;
    for i in 2..value_sqrt {
        if value % i as f32 == 0.0 {
            return false;
        }
    }
    return true;
}
