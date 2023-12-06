
use crate::util::{read_file_as_text, split_numbers_by_space};
use std::io::prelude::*;

type Milisecond = usize;
type Milimeter = usize;

type Race = (Milisecond, Milimeter); // Race(time_in_ms, record_in_milimeters)

fn parse_races(time_line: &str, distance_line: &str) -> Vec<Race> {
    let times = split_numbers_by_space(&time_line[9..]).into_iter();
    let distances = split_numbers_by_space(&distance_line[9..]).into_iter();
    
    times.zip(distances).map(|(t, d)| (t, d)).collect()
}

fn parse_races2(time_line: &str, distance_line: &str) -> Vec<Race> {
    let times = split_numbers_by_space(&time_line[9..].replace(" ", "")).into_iter();
    let distances = split_numbers_by_space(&distance_line[9..].replace(" ", "")).into_iter();
    
    times.zip(distances).map(|(t, d)| (t, d)).collect()
}

fn distance_for_button_held(race_time: Milisecond, button_held_for_ms: Milisecond) -> Milimeter {
    let remaining = race_time.saturating_sub(button_held_for_ms);

    // 1 ms held = 1 mm
    let speed = remaining * button_held_for_ms;
    speed    
}

fn victories_for_each_record(race: Race) -> Vec<Milimeter> {
    let (time, record_distance) = race;

    (0..=time).map(|n| distance_for_button_held(time, n)).filter(|&n| n > record_distance).collect()
}

/*
pub fn day6p1() {
    let game_file = read_file_as_text("./inputs/day6real.txt");
    //let game_file = read_file_as_text("./inputs/day6test.txt");

    let mut lines = game_file.lines().take(2);
    let races = parse_races(&lines.next().unwrap().unwrap(),
                            &lines.next().unwrap().unwrap());
    
    println!("Hello {:?}", races);
    let victories: usize = races.iter().map(|r| {
        let vs = victories_for_each_record(r.clone());
        vs.len()
    }).inspect(|v| println!("{}", v)).product();

    println!("\n{}", victories);
}
*/

pub fn day6() {
    let game_file = read_file_as_text("./inputs/day6real.txt");
    //let game_file = read_file_as_text("./inputs/day6test.txt");

    let mut lines = game_file.lines().take(2);
    let races = parse_races2(&lines.next().unwrap().unwrap(),
                             &lines.next().unwrap().unwrap());
    
    println!("Hello {:?}", races);
    let victories: usize = races.iter().map(|r| {
        let vs = victories_for_each_record(r.clone());
        vs.len()
    }).inspect(|v| println!("{}", v)).product();

    println!("\n{}", victories);
}
