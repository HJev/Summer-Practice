use std::io;
fn main() {
    let mut state: Option<String> = None;
    let mut prev_note: Option<usize> = None;
    let asc: String = String::from("ascedning");
    //   let des : String = String::from("descedning");
    for _i in 0..8 {
        let mut user_input = String::new();

        //println!("Enter a number greater than 3");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read number!");

        let cur_note: usize = user_input.trim().parse().expect("Failed to parse number");
        // behold match statement
        // in the future id like to find a way to cut down on the if and else if parts
        match prev_note {
            None => {prev_note = Some(cur_note)},
            Some(note) => match state {
                None => {
                    if cur_note == note + 1 {
                        state = Some(String::from("ascedning"));
                        prev_note = Some(cur_note);
                    } else if cur_note == note - 1 {
                        state = Some(String::from("descedning"));
                        prev_note = Some(cur_note);
                    } else {
                        state = Some(String::from("mixed"));
                        break;
                    }
                }
                Some(ref cur_state) => match *cur_state == asc {
                    true => {
                        if cur_note == note + 1 {
                            prev_note = Some(cur_note);
                            continue;
                        } else {
                            state = Some(String::from("mixed"));
                            break;
                        }
                    }
                    false => {
                        if cur_note == note - 1 {
                            prev_note = Some(cur_note);
                            continue;
                        } else {
                            state = Some(String::from("mixed"));
                            break;
                        }
                    }
                },
            },
        }
    }
    println!("{}", state.expect("already init"))
}
