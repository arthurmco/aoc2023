mod day1;
mod day2;
mod day3;
pub mod util;

use day1::day1;
use day2::day2;
use day3::day3;

fn main() {
    let day = std::env::args().nth(1).expect("You need to pass the day (in the form of day<X>)");

    match day.as_str() {
        "day1" => day1(),
        "day2" => day2(),
        "day3" => day3(),
        _ => panic!("Day not found!")
    }   
}
