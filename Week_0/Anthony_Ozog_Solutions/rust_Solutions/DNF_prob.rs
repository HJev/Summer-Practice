fn main() {
    let vec_of_nums: Vec<u16> = vec![0, 1, 2, 2, 1, 0, 0, 2, 0, 1, 1, 0];
    let vec_iter = vec_of_nums.iter();
    let mut sorted_vec: Vec<usize> = vec![0; vec_of_nums.len()];

    let mut zeros: usize = 0;
    let mut ones: usize = 0;
    let mut twos: usize = 0;
    for item in vec_iter {
        match item {
            0 => zeros += 1,
            1 => ones += 1,
            2 => twos += 1,
            _ => todo!(),
        };
    }

    macro_rules! sort_builder {
        ($start:expr, $stop:expr, $val:expr, $vector : expr) => {{
            for place in $start..$stop {
                $vector[place] = $val;
            }
            $vector
        }};
    }

    sorted_vec = sort_builder!(zeros, zeros + ones, 1, sorted_vec);
    sorted_vec = sort_builder!(zeros + ones, zeros + ones + twos, 2, sorted_vec);

    println!("the original vector was {:?}", vec_of_nums);
    println!("the sorted vector is {:?}", sorted_vec)
}
