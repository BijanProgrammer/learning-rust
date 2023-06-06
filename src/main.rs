use std::io::{self, Write};

fn main() {
    let mut name = String::new();

    print!("What's your name? ");
    io::stdout().flush().expect("failed to flush");

    io::stdin()
        .read_line(&mut name)
        .expect("failed to read line");

    println!("Hello, {}!", name.trim());
}
