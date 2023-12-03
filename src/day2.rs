use crate::util::read_file_as_text;
use regex::Regex;
use std::cmp;
use std::default::Default;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct CubeSet {
    pub blues: usize,
    pub greens: usize,
    pub reds: usize,
}

impl Default for CubeSet {
    fn default() -> Self {
        CubeSet {
            blues: 0,
            greens: 0,
            reds: 0,
        }
    }
}

impl CubeSet {
    pub fn power(&self) -> usize {
        self.blues * self.greens * self.reds
    }
}

type GameID = usize;

type Game = (GameID, Vec<CubeSet>);

fn parse_game(line: &str) -> GameID {
    // Game ???
    let game_number = &line[4..].trim();
    game_number.parse::<GameID>().unwrap()
}

fn parse_each_set(line: &str) -> CubeSet {
    // 3 X, 4 Y
    let regex = Regex::new(r"(?m)\s*(\d*)\s*([a-z]*)").unwrap();

    line.split(",")
        .into_iter()
        .fold(CubeSet::default(), |acc, cube_line| {
            let caps = regex.captures(cube_line).unwrap();
            let count = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let color = caps.get(2).unwrap().as_str();

            match color {
                "blue" => CubeSet {
                    blues: acc.blues + count,
                    ..acc
                },
                "green" => CubeSet {
                    greens: acc.greens + count,
                    ..acc
                },
                "red" => CubeSet {
                    reds: acc.reds + count,
                    ..acc
                },
                _ => panic!("Color unsupported!"),
            }
        })
}

fn parse_round(line: &str) -> Vec<CubeSet> {
    // 3 X, 4 Y; 1 X, 2 Y...
    line.split(";").into_iter().map(parse_each_set).collect()
}

fn parse_line(line: &str) -> Game {
    let mut game_split = line.split(":");

    let game_id = parse_game(game_split.next().unwrap());
    let game_sets = parse_round(game_split.next().unwrap());

    (game_id, game_sets)
}

pub fn _day2t1() {
    //let game_file = read_file_as_text("./inputs/day2test1.txt");
    let game_file = read_file_as_text("./inputs/day2real.txt");

    let parsed_game = game_file
        .lines()
        .into_iter()
        .map(|v| parse_line(&v.unwrap()))
        .inspect(|e| eprintln!("{:?}", e));
    let id_sum: usize = parsed_game
        .filter(|(_game, sets)| {
            let is_every_round_possible = sets
                .iter()
                .all(|set| set.reds <= 12 && set.greens <= 13 && set.blues <= 14);
            println!("{:?}", is_every_round_possible);
            is_every_round_possible
        })
        .map(|(game, _sets)| game)
        .inspect(|e| eprintln!("<<{}>> ", e))
        .sum();

    println!("\n\n{}", id_sum);
}

pub fn day2() {
    //let game_file = read_file_as_text("./inputs/day2test1.txt");
    let game_file = read_file_as_text("./inputs/day2real.txt");

    let parsed_game = game_file
        .lines()
        .into_iter()
        .map(|v| parse_line(&v.unwrap()));
    let id_sum: usize = parsed_game
        .map(|(_game, sets)| {
            let minimum_set = sets
                .iter()
                .cloned()
                .reduce(|acc, round| CubeSet {
                    blues: cmp::max(acc.blues, round.blues),
                    greens: cmp::max(acc.greens, round.greens),
                    reds: cmp::max(acc.reds, round.reds),
                })
                .unwrap();

            let power = minimum_set.power();
            eprintln!("{:?} {}", minimum_set, power);
            power
        })
        .sum();

    println!("\n\n{}", id_sum);
}
