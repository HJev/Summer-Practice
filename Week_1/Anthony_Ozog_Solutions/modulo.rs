use std::collections::HashMap;
use std::io;
fn main() {
    let mut uniq_dict = HashMap::new();
    for _i in 0..10 {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read number!");

        let cur_number : usize = user_input
            .trim()
            .parse::<usize>()
            .expect("Please enter a valid number!")
            % 42;
        let count = uniq_dict.entry(cur_number).or_insert(0);
        *count += 1;
    }
    let mut uniq_nums : usize = 0;
    for (_,_) in uniq_dict{
        uniq_nums += 1
    }
    println!("{}", uniq_nums);
}
