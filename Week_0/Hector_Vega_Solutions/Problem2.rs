fn dutch(array: &mut [u8]) {
    let (mut z, mut o) = (0, 0);
    for i in 0..array.len() {
        z += if array[i] == 0 { 1 } else { 0 };
        o += if array[i] == 1 { 1 } else { 0 };
    }
    for i in 0..array.len() {
        if i < z {
            array[i] = 0;
        } else if i < o + z {
            array[i] = 1;
        } else {
            array[i] = 2;
        }
    }
}

fn main() {
    let mut arr: [u8; 5] = [1, 2, 1, 1, 0];
    let slice = &mut arr[..];
    printArr(slice);
    dutch(slice);
    printArr(slice);
}

fn printArr(arr: &[u8]) {
    print!("{{ ");
    for i in arr {
        print!("{} ", i);
    }
    println!("}}");
}
