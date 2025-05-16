
fn main() {
    let x = 16;
    let y = 8;
    let gcd = euclidean(x, y);

    println!("{}", gcd);
}

fn euclidean (x: u32, y: u32) -> u32 {
    if x == 0 {
        return y;
    }
    return euclidean(y % x, x);
}
