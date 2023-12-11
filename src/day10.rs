use crate::util::{generate_adjacencies, read_file_as_text};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    PipeVertical,   // north-south
    PipeHorizontal, // east-west
    PipeNorthEast,
    PipeNorthWest,
    PipeWestSouth,
    PipeEastSouth,
    Start,
    Ground,
}

impl Tile {
    fn is_pipe(&self) -> bool {
        match self {
            Self::PipeVertical | Self::PipeHorizontal | Self::PipeNorthEast => true,
            Self::PipeNorthWest | Self::PipeEastSouth | Self::PipeWestSouth => true,
            _ => false,
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Tile::PipeVertical),
            '-' => Ok(Tile::PipeHorizontal),
            'L' => Ok(Tile::PipeNorthEast),
            'J' => Ok(Tile::PipeNorthWest),
            '7' => Ok(Tile::PipeWestSouth),
            'F' => Ok(Tile::PipeEastSouth),
            '.' => Ok(Tile::Ground),
            'S' => Ok(Tile::Start),
            x => Err(format!("Invalid tile: {}", x)),
        }
    }
}

type MapPosition = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

impl Direction {
    fn from_comparation_with_neighbor(current: MapPosition, other: MapPosition) -> Direction {
        let difference = (
            current.0 as isize - other.0 as isize,
            current.1 as isize - other.1 as isize,
        );

        match difference {
            (-1, 0) => Direction::Bottom,
            (1, 0) => Direction::Top,
            (0, -1) => Direction::Right,
            (0, 1) => Direction::Left,

            (-1, 1) => Direction::BottomRight,
            (-1, -1) => Direction::BottomLeft,
            (1, 1) => Direction::TopLeft,
            (1, -1) => Direction::TopRight,

            (0, 0) => panic!("There is no difference"),
            _ => panic!("Impossible!"),
        }
    }
}

#[derive(Debug)]
struct MapNodeElement {
    start_distance: usize,
    connections: Vec<MapPosition>,
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn from_lines(lines: impl Iterator<Item = String>) -> Map {
        let tiles = lines
            .map(|line| {
                line.chars()
                    .filter_map(|c| Tile::try_from(c).ok())
                    .collect()
            })
            .collect();

        Self { tiles }
    }

    /// returns (y, x)
    fn find_start(&self) -> MapPosition {
        self.tiles
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(x, tile)| {
                        if let Tile::Start = tile {
                            return Some(x);
                        } else {
                            return None;
                        }
                    })
                    .map(|x| (y, x))
            })
            .unwrap()
    }

    fn is_connectable_horizontally(&self, current: Tile, other: Tile) -> bool {
        match (current, other) {
            (Tile::Ground, _) | (_, Tile::Ground) => false,
            (Tile::PipeHorizontal, Tile::PipeHorizontal) => true,
            (Tile::Start, x) if (x == Tile::PipeHorizontal) => true,
            (x, Tile::Start)
                if (x == Tile::PipeHorizontal
                    || x == Tile::PipeEastSouth
                    || x == Tile::PipeNorthEast) =>
            {
                true
            }
            (Tile::PipeHorizontal, x) if (x == Tile::PipeWestSouth || x == Tile::PipeNorthWest) => {
                true
            }
            (x, Tile::PipeHorizontal) if (x == Tile::PipeNorthEast || x == Tile::PipeEastSouth) => {
                true
            }
            (Tile::Start, x) if (x == Tile::PipeWestSouth || x == Tile::PipeNorthWest) => true,
            (Tile::PipeNorthEast, Tile::PipeNorthWest) => true,
            (Tile::PipeNorthEast, Tile::PipeWestSouth) => true,
            (Tile::PipeEastSouth, Tile::PipeWestSouth) => true,
            (Tile::PipeEastSouth, Tile::PipeNorthWest) => true,
            _ => false,
        }
    }

    fn is_connectable_vertically(&self, current: Tile, other: Tile) -> bool {
        match (current, other) {
            (Tile::Ground, _) | (_, Tile::Ground) => false,
            (Tile::PipeVertical, Tile::PipeVertical) => true,
            (Tile::Start, x) if (x == Tile::PipeVertical) => true,
            (x, Tile::Start)
                if (x == Tile::PipeVertical
                    || x == Tile::PipeWestSouth
                    || x == Tile::PipeEastSouth) =>
            {
                true
            }
            (Tile::PipeVertical, x) if (x == Tile::PipeNorthEast || x == Tile::PipeNorthWest) => {
                true
            }
            (Tile::Start, x) if (x == Tile::PipeNorthEast || x == Tile::PipeNorthWest) => true,
            (x, Tile::PipeVertical) if (x == Tile::PipeWestSouth || x == Tile::PipeEastSouth) => {
                true
            }
            (Tile::PipeEastSouth, Tile::PipeNorthWest) => true,
            (Tile::PipeWestSouth, Tile::PipeNorthWest) => true,
            (Tile::PipeEastSouth, Tile::PipeNorthEast) => true,
            (Tile::PipeWestSouth, Tile::PipeNorthEast) => true,
            _ => false,
        }
    }

    fn is_connectable(&self, current: Tile, other: Tile, direction: Direction) -> bool {
        match direction {
            Direction::Top => self.is_connectable_vertically(other, current),
            Direction::Bottom => self.is_connectable_vertically(current, other),
            Direction::Left => self.is_connectable_horizontally(other, current),
            Direction::Right => self.is_connectable_horizontally(current, other),
            _ => false,
        }
    }

    /// Only check if the neighbors of a certain tile connects
    fn what_neighbors_connects_to(&self, pos: MapPosition) -> Vec<MapPosition> {
        let (y, x) = pos;
        let adjs = generate_adjacencies(&self.tiles, x, y);

        adjs.into_iter()
            .filter(|npos| {
                let (ny, nx) = npos;
                let direction = Direction::from_comparation_with_neighbor((y, x), (*ny, *nx));
                self.is_connectable(self.tiles[y][x], self.tiles[*ny][*nx], direction)
            })
            .collect()
    }

    fn create_node_element_position(
        &self,
        start_distance: usize,
        position: MapPosition,
    ) -> (MapPosition, MapNodeElement) {
        let connections = self.what_neighbors_connects_to(position);

        (
            position,
            MapNodeElement {
                start_distance,
                connections,
            },
        )
    }

    fn retrieve_map_distances(&self) -> HashMap<MapPosition, MapNodeElement> {
        let start = self.find_start();
        let mut pos_queue: VecDeque<(MapPosition, MapNodeElement)> = VecDeque::new();
        let mut ret: HashMap<MapPosition, MapNodeElement> = HashMap::new();
        let mut visited: HashSet<MapPosition> = HashSet::new();

        pos_queue.push_back(self.create_node_element_position(0, start));
        visited.insert(start);

        while !pos_queue.is_empty() {
            let node = pos_queue.pop_front().unwrap();
            let distance = node.1.start_distance;

            for conn in node.1.connections.iter() {
                if visited.contains(conn) {
                    continue;
                }

                pos_queue.push_back(self.create_node_element_position(distance + 1, *conn));
                visited.insert(*conn);
            }

            //eprintln!("-- {:?} {:?}", node.0, node.1);
            ret.insert(node.0, node.1);
        }

        ret
    }

    fn retrieve_map_farthest(&self, posmap: &HashMap<MapPosition, MapNodeElement>) -> usize {
        let (_, element) = posmap
            .iter()
            .max_by(|&(_, e1), &(_, e2)| e1.start_distance.cmp(&e2.start_distance))
            .unwrap();

        element.start_distance
    }
}

// 13639 = not
// 15
pub fn day10() {
    let game_file = read_file_as_text("./inputs/day10real.txt").lines();
    //let game_file = read_file_as_text("./inputs/day10test3.txt").lines();
    let map = Map::from_lines(game_file.filter_map(|s| s.ok()));
    let distances = map.retrieve_map_distances();

    //println!("Hello {:?}", map);
    println!("start @ {:?}", map.find_start());
    println!(
        "conn @ {:?}",
        map.what_neighbors_connects_to(map.find_start())
    );
    //println!("distances @ {:?}", distances);
    println!("\n{}", map.retrieve_map_farthest(&distances))
}
