use crate::util::read_file_as_text;
use regex::Regex;
use std::default::Default;
use std::io::prelude::*;
use std::ops::Add;

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

impl Add for CubeSet {
    type Output = CubeSet;

    fn add(self, rhs: Self) -> Self::Output {
        CubeSet {
            blues: self.blues + rhs.blues,
            greens: self.greens + rhs.greens,
            reds: self.reds + rhs.reds,
        }
    }
}

type GameID = usize;

type Game = (GameID, Vec<CubeSet>);

fn parse_game(line: &str) -> GameID {
    // Game ???
    let game_number = &line[4..].trim();
    game_number.parse::<GameID>().unwrap()
}

fn parse_each_round(line: &str) -> CubeSet {
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

fn parse_sets(line: &str) -> Vec<CubeSet> {
    // 3 X, 4 Y; 1 X, 2 Y...
    line.split(";").into_iter().map(parse_each_round).collect()
}

fn parse_line(line: &str) -> Game {
    let mut game_split = line.split(":");

    let game_id = parse_game(game_split.next().unwrap());
    let game_sets = parse_sets(game_split.next().unwrap());

    (game_id, game_sets)
}

pub fn day2() {
    //let game_file = read_file_as_text("./inputs/day2test1.txt"); 
    let game_file = read_file_as_text("./inputs/day2real.txt");
    
    let parsed_game = game_file
        .lines()
        .into_iter()
        .map(|v| parse_line(&v.unwrap()))
        .inspect(|e| eprintln!("{:?}", e));    
    let id_sum: usize = parsed_game
        .filter(|(_game, sets)| {
            let is_every_round_possible = sets.iter().all(
                |set| set.reds <= 12 && set.greens <= 13 && set.blues <= 14
            );
            println!("{:?}", is_every_round_possible);
            is_every_round_possible
        })
        .map(|(game, _sets)| game)
        .inspect(|e| eprintln!("<<{}>> ", e))
        .sum();

    println!("\n\n{}", id_sum);
}
