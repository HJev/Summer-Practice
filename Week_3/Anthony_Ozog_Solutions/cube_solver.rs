use std::io;

fn mtx_maker(A : f64) -> Vec<Vec<f64>>{
    
    let tw : f64 = 1.0/12.0;
    let rt : f64 = (6.0 - 2.0 * (A).powf(2.0)).powf(0.5);
    vec![ 
        vec![tw *(3.0 + A + rt), tw * (-2.0 * A + rt), tw* (3.0 - A - rt) ],
        vec![tw *(2.0*A - rt), tw * 2.0 * (A + rt), tw* ( -2.0* A + rt)   ],
        vec![tw *(3.0 - A - rt), tw * (2.0 * A - rt), tw* (3.0 + A + rt) ],
    ]
    
}


fn print_my_mtx(mtx: Vec<Vec<f64>>) {
    for sub_vec in mtx {
        println!("{:?}", sub_vec);
    }
}



fn main() {

    let mut first_line = String::new();

    io::stdin()
        .read_line(&mut first_line)
        .expect("Failed to read line");

    let n: usize = first_line.trim().parse().expect("Please type a number!");

    let mut cases: Vec<f64> = vec![0.0; n];

    for i in 0..n {
        let mut cur_line = String::new();

        io::stdin()
            .read_line(&mut cur_line)
            .expect("Failed to read line");

        cases[i] = cur_line
            .trim()
            .parse()
            .expect("Please enter data correctly")
    }

    for case in cases.iter() {
    

        let sol = mtx_maker(*case);
        print_my_mtx(sol)
    }
}
