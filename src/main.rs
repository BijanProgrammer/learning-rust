use std::io::{self, Write};

fn main() {
    let mut user_input = String::new();

    // NOTE: [Bijan] Getting the first degree
    print!("What's the first angle? ");
    io::stdout().flush().expect("Failed to flush.");

    user_input.clear();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line.");

    let first: i32 = user_input
        .trim()
        .parse()
        .expect("Failed to parse into i32 number.");

    // NOTE: [Bijan] Getting the second degree
    print!("What's the second angle? ");
    io::stdout().flush().expect("Failed to flush.");

    user_input.clear();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line.");

    let second: i32 = user_input
        .trim()
        .parse()
        .expect("Failed to parse into i32 number.");

    // NOTE: [Bijan] Getting the third degree
    print!("What's the third angle? ");
    io::stdout().flush().expect("Failed to flush.");

    user_input.clear();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line.");

    let third: i32 = user_input
        .trim()
        .parse()
        .expect("Failed to parse into i32 number.");

    // NOTE: [Bijan] Printing the result
    if first + second + third == 180 {
        println!("You can make a triangle using these angles.");
    } else {
        println!("You cannot make a triangle using these angles.");
    }
}
