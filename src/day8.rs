use crate::util::{read_file_as_text};
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            x => Err(format!("Invalid direction: {}", x)),
        }
    }
}


type MapNode = String;
type MapIterResult = (MapNode, Direction);
type MapPoint = (MapNode, MapNode);

#[derive(Debug)]
struct Map {
    directions: Vec<Direction>,
    current_direction_index: usize,
    current_node: MapNode,
    nodes: HashMap<MapNode, MapPoint>
}

impl Map {
    fn parse_directions(line: &str) -> Vec<Direction> {
        line.chars().filter_map(|c| Direction::try_from(c).ok() ).collect()
    }

    fn parse_nodes(lines: impl Iterator<Item=String>) -> HashMap<MapNode, MapPoint> {
        let regex = Regex::new(r"(?m)\s*([A-Z]+)\s+=\s+\(([A-Z]+),\s*([A-Z]+)\)").unwrap();
        
        let mut map = HashMap::new();
        for l in lines {
            let caps = regex.captures(&l).unwrap();
            let node = caps.get(1).unwrap().as_str().to_string();
            let left = caps.get(2).unwrap().as_str().to_string();
            let right = caps.get(3).unwrap().as_str().to_string();

            map.insert(node, (left, right));
        }

        map
    }

    fn new(mut lines: impl Iterator<Item=String>) -> Self {
        let directions = Map::parse_directions(&lines.next().unwrap());
        let empty_line = lines.next().unwrap();

        if !empty_line.trim().is_empty() {
            panic!("empty line not found... why?");
        }

        let nodes = Map::parse_nodes(lines);

        Map {
            directions,
            current_direction_index: 0,
            current_node: String::from("AAA"),
            nodes
        }
    }

    fn reset(&mut self) {
        self.current_node = String::from("AAA");
        self.current_direction_index = 0;
    }

    fn increase_direction(&self) -> usize {
        let result = self.current_direction_index + 1;
        if result >= self.directions.len() {
            0
        } else {
            result
        }
    }
    
    fn iterate_node(&mut self) -> Option<MapIterResult> {
        if self.current_node == "ZZZ" {
            None
        } else {
            let direction = self.directions[self.current_direction_index];            
            let (path_left, path_right) = self.nodes.get(&self.current_node).unwrap();
            let next_node = match direction {
                Direction::Left => path_left,
                Direction::Right => path_right
            };

            self.current_direction_index = self.increase_direction();
            self.current_node = next_node.clone();
            Some((next_node.clone(), direction))
        }
    }
}


pub fn day8() {
    let game_file = read_file_as_text("./inputs/day8real.txt").lines();    
    let mut map = Map::new(game_file.map(|s| s.unwrap()));
    
    println!("Hello, {:?}", map);

    let mut nums = 0;
    while let Some((node, dir)) = map.iterate_node() {
        println!("{} {:?}", node, dir);
        nums += 1;
    }

    println!("\n{}", nums);
}
