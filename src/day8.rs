#![allow(dead_code, unused_mut, unused_variables)]

use crate::util::read_file_as_text;
use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::iter::FusedIterator;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
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
    nodes: HashMap<MapNode, MapPoint>,
}

impl Map {
    fn parse_directions(line: &str) -> Vec<Direction> {
        line.chars()
            .filter_map(|c| Direction::try_from(c).ok())
            .collect()
    }

    fn parse_nodes(lines: impl Iterator<Item = String>) -> HashMap<MapNode, MapPoint> {
        let regex = Regex::new(r"(?m)\s*([A-Z0-9]+)\s+=\s+\(([A-Z0-9]+),\s*([A-Z0-9]+)\)").unwrap();

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

    fn new(mut lines: impl Iterator<Item = String>) -> Self {
        let directions = Map::parse_directions(&lines.next().unwrap());
        let empty_line = lines.next().unwrap();

        if !empty_line.trim().is_empty() {
            panic!("empty line not found... why?");
        }

        let nodes = Map::parse_nodes(lines);

        Map { directions, nodes }
    }

    fn increase_direction(&self, current_direction: usize) -> usize {
        let result = current_direction + 1;
        if result >= self.directions.len() {
            0
        } else {
            result
        }
    }

    fn simple_iter(&self) -> SimpleMapIter {
        SimpleMapIter::new(self)
    }

    fn simple_iter_with_custom_start(&self, start: &str) -> SimpleMapIter {
        SimpleMapIter::new_with_custom_start(self, start)
    }

    fn ghost_iter(&self) -> GhostMapIter {
        GhostMapIter::new(self)
    }
}

struct SimpleMapIter<'a> {
    map: &'a Map,
    current_direction_index: usize,
    current_node: MapNode,
    is_second_part: bool,
}

impl<'a> SimpleMapIter<'a> {
    fn new(map: &'a Map) -> Self {
        SimpleMapIter {
            map,
            current_direction_index: 0,
            current_node: String::from("AAA"),
            is_second_part: false,
        }
    }

    fn new_with_custom_start(map: &'a Map, start_node: &str) -> Self {
        SimpleMapIter {
            map,
            current_direction_index: 0,
            current_node: start_node.to_string(),
            is_second_part: true,
        }
    }

    fn is_end_node(&self) -> bool {
        match self.is_second_part {
            false => self.current_node == "ZZZ",
            true => self.current_node.ends_with('Z'),
        }
    }
}

impl<'a> Iterator for SimpleMapIter<'a> {
    type Item = MapIterResult;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end_node() {
            None
        } else {
            let direction = self.map.directions[self.current_direction_index];
            let (path_left, path_right) = self.map.nodes.get(&self.current_node).unwrap();
            let next_node = match direction {
                Direction::Left => path_left,
                Direction::Right => path_right,
            };

            self.current_direction_index =
                self.map.increase_direction(self.current_direction_index);
            self.current_node = next_node.clone();
            Some((next_node.clone(), direction))
        }
    }
}

impl<'a> FusedIterator for SimpleMapIter<'a> {}

struct GhostMapIter<'a> {
    map: &'a Map,
    current_direction_index: usize,
    current_nodes: Vec<String>,
}

/* extremely inefficient... but cool if you wanna use iterators, or if you
 * want to check if this works (for lower values), or even if you want to make a
 * visualizer */
impl<'a> GhostMapIter<'a> {
    fn new(map: &'a Map) -> Self {
        let current_nodes = map
            .nodes
            .keys()
            .filter(|n| n.ends_with('A'))
            .cloned()
            .collect();
        GhostMapIter {
            map,
            current_direction_index: 0,
            current_nodes,
        }
    }

    fn is_end_node_for_all(&self) -> bool {
        self.current_nodes.iter().all(|n| n.ends_with('Z'))
    }
}

impl<'a> Iterator for GhostMapIter<'a> {
    type Item = Vec<MapIterResult>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end_node_for_all() {
            None
        } else {
            let next_nodes: Vec<MapIterResult> = self
                .current_nodes
                .iter()
                .map(|n| {
                    let direction = self.map.directions[self.current_direction_index];
                    let (path_left, path_right) = self.map.nodes.get(n).unwrap();
                    let next_node = match direction {
                        Direction::Left => path_left,
                        Direction::Right => path_right,
                    };

                    (next_node.clone(), direction)
                })
                .collect();

            self.current_direction_index =
                self.map.increase_direction(self.current_direction_index);
            self.current_nodes = next_nodes.iter().map(|(n, _)| n).cloned().collect();
            Some(next_nodes)
        }
    }
}

impl<'a> FusedIterator for GhostMapIter<'a> {}

// faster way to do this, use this to get the answer in your lifetime lol
fn faster_ghost_map_count(map: &Map) -> usize {
    let current_nodes: Vec<String> = map
        .nodes
        .keys()
        .filter(|n| n.ends_with('A'))
        .cloned()
        .collect();

    current_nodes
        .into_iter()
        .map(|node| map.simple_iter_with_custom_start(&node).count())
        .reduce(lcm)
        .unwrap()
}

pub fn day8() {
    let game_file = read_file_as_text("./inputs/day8real.txt").lines();
    let map = Map::new(game_file.map(|s| s.unwrap()));

    println!("Hello, {:?}", map);

    let nums = map
        .simple_iter()
        .inspect(|(node, dir)| println!("{} {:?}", node, dir))
        .count();
    println!("\n{}", nums);
}

use console_engine::pixel;
use console_engine::screen::Screen;
use console_engine::Color;
use std::{thread, time};

fn get_x1_coord_to_center(width: usize, value: &str) -> usize {
    (width / 2) - (value.chars().count() / 2)
}

fn get_x2_coord_to_center(width: usize, value: &str) -> usize {
    (width / 2) + (value.chars().count() / 2)
}

fn draw_screens(
    graph: &Screen,
    icon: &Screen,
    dirs: &Screen,
    engine: &mut console_engine::ConsoleEngine,
) {
    engine.rect(50, 0, 50, 18, pixel::pxl('|'));

    engine.print_screen(0, 0, graph);
    engine.print_screen(53, 2, icon);
    engine.print_screen(0, 19, dirs);
    engine.draw();

    engine.wait_frame();
    engine.clear_screen();
}

fn draw_directions(scr: &mut Screen, mut dirs: impl Iterator<Item = Direction>) {
    scr.clear();

    let first = dirs.next().unwrap();
    let text = format!("  {:?}  ", first);
    let x1 = get_x1_coord_to_center(13, &text);
    scr.print_fbg(x1 as i32, 0, &text, Color::Black, Color::White);

    scr.print_fbg(14, 0, "*", Color::White, Color::Reset);

    let mut index = 15;

    for d in dirs {
        let text = format!("{:?}", d);
        let x1 = get_x1_coord_to_center(13, &text);
        scr.print_fbg(index + x1 as i32, 0, &text, Color::White, Color::Reset);

        scr.print_fbg(index + 14, 0, "*", Color::White, Color::Reset);
        index += 15
    }
}

pub fn day8view() {
    let game_file = read_file_as_text("./inputs/day8real.txt").lines();
    let map = Map::new(game_file.map(|s| s.unwrap()));

    let mut engine = console_engine::ConsoleEngine::init(70, 20, 5).unwrap();
    engine.wait_frame();
    engine.clear_screen();

    let mut graph_str = Screen::new(45, 15);
    let mut icon_str = Screen::new(15, 11);
    let mut dir_str = Screen::new(70, 1);

    icon_str.print_fbg(
        0,
        0,
        r#"       .-.
      ( " )
   /\_.' '._/\
   |         |
    \       /
     \    /`
   (__)  /
jgs`.__.'"#,
        Color::Cyan,
        Color::Reset,
    );

    let text = "AAA";
    let x1pos = get_x1_coord_to_center(graph_str.get_width() as usize, text);
    let x2pos = get_x2_coord_to_center(graph_str.get_width() as usize, text);
    graph_str.rect(
        (x1pos as i32) - 2,
        7,
        (x2pos as i32) + 2,
        11,
        pixel::pxl('*'),
    );
    draw_screens(&graph_str, &icon_str, &dir_str, &mut engine);
    graph_str.print_fbg(x1pos as i32, 9, text, Color::White, Color::Reset);

    draw_screens(&graph_str, &icon_str, &dir_str, &mut engine);

    let mut idx = 0;
    for m in map.simple_iter() {
        thread::sleep(time::Duration::from_millis(300));
        draw_directions(
            &mut dir_str,
            map.directions.iter().cycle().skip(idx).take(3).cloned(),
        );

        let (node, dir) = m;
        let direction = match dir {
            Direction::Left => -1,
            Direction::Right => 1,
        };

        for v in 0..5 {
            let dir = match v {
                0..=3 => direction,
                _ => 0,
            };

            graph_str.scroll(direction, 1, pixel::pxl(' '));
            draw_screens(&graph_str, &icon_str, &dir_str, &mut engine);
        }

        let text = node.to_string();
        let x1pos = get_x1_coord_to_center(graph_str.get_width() as usize, &text);
        let x2pos = get_x2_coord_to_center(graph_str.get_width() as usize, &text);

        graph_str.rect(
            (x1pos as i32) - 2,
            7,
            (x2pos as i32) + 2,
            11,
            pixel::pxl('*'),
        );
        draw_screens(&graph_str, &icon_str, &dir_str, &mut engine);
        graph_str.print_fbg(x1pos as i32, 9, &text, Color::White, Color::Reset);
        draw_screens(&graph_str, &icon_str, &dir_str, &mut engine);

        thread::sleep(time::Duration::from_millis(300));
        draw_screens(&graph_str, &icon_str, &dir_str, &mut engine);
        idx += 1;
    }

    //let nums = map.simple_iter().inspect(|(node, dir)| println!("{} {:?}", node, dir)).count();
    //println!("\n{}", nums);
}

pub fn day8t2() {
    let game_file = read_file_as_text("./inputs/day8real.txt").lines();
    let map = Map::new(game_file.map(|s| s.unwrap()));

    println!("Hello, {:?}", map);

    //let nums = map.ghost_iter().inspect(|nodes| println!("{:?}", nodes)).count();
    //println!("\n{}", nums);

    let nums = faster_ghost_map_count(&map);
    println!("\n{}", nums);
}
