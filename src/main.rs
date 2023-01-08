use std::env;

pub mod day1;
pub mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].as_str();

    match day {
        "day1" => day1::run(),
        "day2" => day2::run(),
        &_ => println!("Invalid day!"),
    }
}
