use std::io;

fn main() {
    println!("Input:");
    let mut input: String = String::new();

    io::stdin().read_line(&mut input).expect("error dumbass");
    let mut size = input.trim().parse::<u16>().unwrap();

    let mut num: i32 = 0;
    for _i in 0..size {
        println!("num:");
        let mut temp: String = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("error again dumbass");
        num += temp.trim().parse::<i32>().unwrap();
    }
    println!("output:\n{}", num);
}
