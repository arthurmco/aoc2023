mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
pub mod util;

use day1::day1;
use day10::day10;
use day11::day11;
use day12::day12;
use day2::day2;
use day3::day3;
use day4::day4;
use day5::day5;
use day6::day6;
use day7::day7;
use day8::{day8, day8view};
use day9::day9;

fn main() {
    let day = std::env::args()
        .nth(1)
        .expect("You need to pass the day (in the form of day<X>)");

    match day.as_str() {
        "day1" => day1(),
        "day2" => day2(),
        "day3" => day3(),
        "day4" => day4(),
        "day5" => day5(),
        "day6" => day6(),
        "day7" => day7(),
        "day8" => day8(),
        "day8view" => day8view(),
        "day9" => day9(),
        "day10" => day10(),
        "day11" => day11(),
        "day12" => day12(),
        _ => panic!("Day not found!"),
    }
}
