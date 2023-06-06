use std::io::{self, Write};

fn main() {
    let first = read_angle(1).unwrap();
    let second = read_angle(2).unwrap();
    let third = read_angle(3).unwrap();

    if first + second + third == 180 {
        println!("You can make a triangle using these angles.");
    } else {
        println!("You cannot make a triangle using these angles.");
    }
}

fn read_angle(angle_number: i32) -> Result<i32, String> {
    let angle_text = convert_angle_number_to_text(angle_number).unwrap();
    print_asap(format!("What's the {angle_text} angle? "));

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line.");

    let angle: i32 = user_input
        .trim()
        .parse()
        .expect("Failed to parse into i32 number.");

    return Ok(angle);
}

fn convert_angle_number_to_text(angle_number: i32) -> Result<String, String> {
    match angle_number {
        1 => return Ok("first".to_owned()),
        2 => return Ok("second".to_owned()),
        3 => return Ok("third".to_owned()),
        _ => return Err(format!("{} is not a valid angle number.", angle_number)),
    }
}

fn print_asap(text: String) -> () {
    print!("{}", text);
    io::stdout().flush().expect("Failed to flush.");
}
