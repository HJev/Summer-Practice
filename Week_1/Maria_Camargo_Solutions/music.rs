use std::io;

fn main() {
    let mut notes = String::new();

    // get user input
    io::stdin()
        .read_line(&mut notes)
        .expect("Failed to read line");

    // take away white space
    let notes = notes.trim();
    let notes = notes.replace(" ", "");

    // collect chars into a vector
    let letters: Vec<char> = notes.chars().collect();

    println!("{}", order(letters));
}

fn order(letters: Vec<char>) -> String {
    let mut result = String::new();
    // loop through vector
    let mut pre = letters[0] as i32 - 0x30;
    let mut asc = 0;
    let mut des = 0;
    for i in letters {
        // convert to digit
        let mut current = i as i32 - 0x30;

        // keep track of asc and des numbers
        if pre < current {
            asc += 1;
        } else if pre > current {
            des += 1;
        }

        // if there are both des and asc digits, it is mixed
        if des != 0 && asc != 0 {
            result.push_str("mixed");
            return result;       
        }

        pre = current;
    }

    if asc == 0 {
        result.push_str("descending");
    } else {
        result.push_str("ascending");
    }

    return result;
}