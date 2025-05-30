
// I am gonna use the sieve method

use std::io;


fn main() {

    let mut user_input = String::new();
    
    println!("Enter a number greater than 3");
    
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read number!");

    let main_num : usize = user_input.trim().parse().expect("Failed to parse number");

    let mut bool_primes : Vec<bool> = vec![true; main_num];

    let mut numbers : Vec<usize> = vec![0; main_num];
    // initalize my numbers
    for i in 1..main_num{
        numbers[i] = numbers[i-1] + 1;
    }

    for pos in 0..main_num {
        if pos == 0 || pos == 1{
            continue
        }
        if bool_primes[pos]{
            let mut new_pos = numbers[pos];
            loop {
                new_pos += numbers[pos];
                if new_pos > main_num - 1{
                    break
                }
                //println!("{} is being marked false", new_pos);
                bool_primes[new_pos] = false; 
            }
        }
    }
    let mut primes : Vec<usize> = vec![];
    for pos in 2..main_num{
        if  bool_primes[pos]{
            primes.push(numbers[pos])
        }
    }


    //println!("{:?}", bool_primes);
    println!("{:?}", primes);
}
