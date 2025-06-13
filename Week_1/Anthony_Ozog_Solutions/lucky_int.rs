// sol is pretty easy, just convert to binary then turn 1s to 7s and 0s to 4s
// this first idea does not work
// it did work, i just had to mod it
// 1  4 1
// 2  7 10
// 3  44 11
// 4  47 100
// 5  74`101`
// 6  77 110
// 7  444 111
// 8  447 1000
// 9  474 1001
//  this way is better so add 1
// 1  4 10
// 2  7 11
// 3  44 100
// 4  47 101
// 5  74`110`
// 6  77 111
// 7  444 1000
// 8  447 1001
// 9  474 1010
use std::io;
fn main() {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read number!");

    let main_number: usize = user_input
        .trim()
        .parse::<usize>()
        .expect("Please enter a valid number!")
        + 1;

    let bin_num : String = format!("{:b}", main_number);
    let mut decomp_vec: Vec<u8> = Vec::new();
    // here we go from bits to 4s and 7s
    for i in bin_num.chars() {
        match i {
            '0' => decomp_vec.push(4),
            '1' => decomp_vec.push(7),
            _ => todo!(),
        }
    }
    let concatenated: &String = &decomp_vec[1..].iter() // skip the first bit
        .map(|&num| num.to_string()) // Convert each u8 to a String
        .collect(); // Collect to String
    
    // same thing as before
    let concatenated_number: usize = concatenated.parse::<usize>().expect("Failed to parse number");

    println!("Lucky Number: {}", concatenated_number);

}
