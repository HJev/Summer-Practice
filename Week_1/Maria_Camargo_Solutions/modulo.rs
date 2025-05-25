/*
Write a program that accepts 10 numbers as input, 
calculates each number modulo 42, and prints the 
count of distinct remainders obtained.
*/
use std::io;

fn main() {
    let mut number = String::new();
    let mut numbers: Vec<u32> = vec![];
    let mut remainders: u32 = 0;

    // get user input
    for _i in 0..10 {
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        // convert string to int
        let mut temp: u32 = number.trim().parse().expect("Please type a number!");
        temp = temp % 42;
        
        if !(numbers.contains(&temp)) {
            // increment count
            remainders += 1;
            // append remainder to list
            numbers.push(temp);
        }
        
        // empty buffer
        number = String::new();
    }

    println!("{remainders}");
}
