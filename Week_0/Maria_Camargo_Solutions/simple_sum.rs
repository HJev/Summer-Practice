use std::io::{self, Stdin};

fn main() {
    let mut num_elements = String::new();
    let element = String::new();
    let mut total = 0;

    io::stdin()
        .read_line(&mut num_elements)
        .expect("Failed to read line");

    let num_elements: u32 = num_elements.trim().parse().expect("Please type a number!");

    let mut i = 0;
    while i < num_elements {
        let mut element = String::new();
        io::stdin()
            .read_line(&mut element)
            .expect("Failed to read line");

        let element: u32 = element.trim().parse().expect("Please type a number");

        total += element;
        i += 1;
    }
    println!("Sum {}", total);
}
