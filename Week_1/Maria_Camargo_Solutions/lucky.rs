use std::io;
fn main() {
    // get user input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut input:u32 = input.trim().parse().expect("enter an integer");

    // calculate the length of the lucky number
    let mut len:u32 = 1;
    let mut count: u32 = 0;
    loop {
        // calc the number of lucky number with length len
        let mut current = 2_u32.pow(len);
        // if input is inside the set of lucky numbers, break
        if count + current >= input {
            break;
        }
        // increment counters
        count += current;
        len += 1;
    }

    // get position in the given length 
    input = input - count - 1;

    // convert to binary
    let mut bin_input = String::from(format!("{input:b}"));
    let bin_input: u32 = bin_input.parse().expect("input needs to be an int");

    // fill left with zeros depending on length
    let len:usize = len.try_into().unwrap();
    let mut bin_input:String = format!("{:0fill$}", bin_input, fill = len);

    // change 0 to 4 and 1 to 7
    bin_input = bin_input.replace("0", "4");
    bin_input = bin_input.replace("1", "7");

    // print number
    println!("{bin_input}");
}

/*
1    4    0
2    7    1
3   44   00
4   47   01
5   74   10
6   77   11
7  444
8  447
9  474
10 477
11 744
12 747
13 774
14 777
*/

