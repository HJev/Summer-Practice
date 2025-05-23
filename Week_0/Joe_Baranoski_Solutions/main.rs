use std::io;

fn main() {
    let run_simple_sum = true;
    let run_dutch = true;
    let run_euclidean = true;


    //Simple Sum
    if run_simple_sum {
        println!("-----Simple Sum------");
        println!("How many numbers would you like to sum?");

        let mut num = String::new(); //initializes the number of entries to be input

        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line"); //gets user input

        let mut num: u32 = match num.trim().parse() { 

            Ok(n) => n, //If it can be converted to a number, convert it
            Err(_) => 0, //If an error is thrown, return 0 instead.

        };
        
        let mut sum = 0;

        while num > 0 { 
            let mut x = String::new(); //each individual number

            println!("Input a number.");

            io::stdin()
                .read_line(&mut x)
                .expect("Failed to read line");

            let x: u32 = match x.trim().parse() {

                Ok(n) => n, //If it can be converted to a number, convert it
                Err(_) => 0, //If an error is thrown, return 0 instead.

            };

            sum += x;
            num -= 1;

        }

        println!("The sum of the numbers is {sum}.");
        println!("\n");

    }
    
    //Dutch National Flag Problem
    if run_dutch {
        const A: [i32; 12] = [0, 1, 2, 2, 1, 0, 0, 2, 0, 1, 1, 0]; 
        let mut sorted = [0;A.len()];

        let mut num_zeros = 0;
        let mut num_ones = 0;

        for i in 0..A.len() {
            match A[i] {
                0 => num_zeros += 1,
                1 => num_ones += 1,
                _ => continue,
            }
        }

        for i in num_zeros..num_ones + num_zeros {
            sorted[i] = 1;
        }
        for i in num_ones + num_zeros..sorted.len() {
            sorted[i] = 2;
        }
        
        println!("The sorted list is {sorted:?}");

    }



    //Euclidean Algorithm
    if run_euclidean {
        println!("-----Euclidean Algorithm-----");

        println!("Input the first number:");

        let mut x = String::new();

        io::stdin()
            .read_line(&mut x)
            .expect("Failed to read line");

        let mut x: u32 = match x.trim().parse() {

            Ok(n) => n, //If it can be converted to a number, convert it
            Err(_) => 0, //If an error is thrown, return 0 instead.

        };

        println!("Input the second number:");

        let mut y = String::new();

        io::stdin()
            .read_line(&mut y)
            .expect("Failed to read line");

        let mut y: u32 = match y.trim().parse() {

            Ok(n) => n, //If it can be converted to a number, convert it
            Err(_) => 0, //If an error is thrown, return 0 instead.

        };

        let mut z = x % y;

        //The actual Euclidean Algorithm
        while z != 0 {
            x = y;
            y = z;
            z = x % y;
        }

        println!("The greatest common divisor is {y}");
    }
    





}

