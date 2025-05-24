fn GCD(num1: u16, num2: u16) -> u16 {
    let r = num1 % num2;

    if r == 0 {
        return num1 / num2;
    }

    return GCD(num2, num1 % num2);
}

fn main() {
    println!("GCD of 104 and 56: {}", GCD(104, 56));
}
