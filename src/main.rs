mod day1;

use day1::day1;


fn main() {
    let day = std::env::args().nth(1).expect("You need to pass the day (in the form of day<X>)");

    match day.as_str() {
        "day1" => day1(),
        _ => panic!("Day not found!")
    }   
}
