fn main() {
    // vars
    let original: [u32; 12] = [ 0, 1, 2, 2, 1, 0, 0, 2, 0, 1, 1, 0];
    let mut ordered: [u32; 12] = [0; 12];
    let mut zeros = 0;
    let mut ones = 0;
    let len = original.len();

    // count numbers
    let mut i = 0;
    for element in original {
        if element == 0 {
            zeros += 1;
        }
        else if element == 1 {
            ones += 1;
        }

        i += 0;
    }

    // insert ones 
    i = zeros;
    while i < (zeros + ones) {
        ordered[i] = 1;
        i += 1;
    }

    // insert twos
    i = zeros + ones;
    while i < len {
        ordered[i] = 2;
        i += 1;
    }

    // print final array
    for element in ordered {
        print!("{} ", element);
    }
    println!("");
}
