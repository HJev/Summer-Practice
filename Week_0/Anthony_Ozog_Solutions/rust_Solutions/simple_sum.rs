use std::io;

fn main() {
    let mut first_line = String::new();

    io::stdin()
        .read_line(&mut first_line)
        .expect("Failed to read line");



    let n: usize = first_line.trim().parse().expect("Please type a number!");


    let mut sum : usize = 0;
    for _ in 0..n {
        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        sum += line.trim().parse::<usize>().expect("Please type a number!");
    }
    println!("{sum}")
    
}
