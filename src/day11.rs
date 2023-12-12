use crate::util::{generate_adjacencies, read_file_as_text};
use std::collections::{HashSet, HashMap};
use std::io::prelude::*;
use itertools::Itertools;

type GalacticPosition = (usize, usize);
type GalacticIndex = usize;

type GalaxyPair = (GalacticIndex, GalacticIndex, usize);

#[derive(Debug)]
struct GalacticMap {
    galaxies: HashMap<GalacticIndex, GalacticPosition>,
}

impl GalacticMap {
    fn find_axis_without_galaxies<F: Fn(&GalacticPosition) -> usize>(
        axis_filter: F,
        gals: &Vec<GalacticPosition>,
    ) -> Vec<usize> {
        let gal_rows = gals.iter().map(axis_filter).collect::<HashSet<usize>>();
        let ymax = *gal_rows.iter().max().unwrap();

        (0..=ymax).filter(|num| !gal_rows.contains(&num)).collect()
    }

    fn transform_space_expansion(scale: usize, axis_without_gal: Vec<usize>) -> Vec<usize> {
        let axis_max: usize = axis_without_gal.iter().max().cloned().unwrap() + 2;

        (0..axis_max)
            .scan(0, |acc, index| {
                let a = *acc;
                if axis_without_gal.contains(&index) {
                    let scale = 1 * (scale - 1);
                    *acc = a + scale;
                    Some(a + scale)
                } else {
                    Some(a)
                }
            })
            .collect::<Vec<usize>>()
    }
    
    fn find_distance(&self, from: GalacticIndex, to: GalacticIndex) -> GalaxyPair {
        let (y1, x1) = self.galaxies.get(&from).unwrap();
        let (y2, x2) = self.galaxies.get(&to).unwrap();

        let manhattan_distance = (x1.abs_diff(*x2)) + (y1.abs_diff(*y2));
        (from, to, manhattan_distance)
    }
    
    fn from_lines(scale: usize, lines: impl Iterator<Item = String>) -> GalacticMap {
        let galaxies = lines
            .enumerate()
            .map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .filter_map(move |(x, c)| match c {
                        '#' => Some((y, x)),
                        _ => None,
                    })
                    .collect::<Vec<GalacticPosition>>()
            })
            .flatten()
            .collect();

        let rows = GalacticMap::transform_space_expansion(scale, GalacticMap::find_axis_without_galaxies(
            |(y, _)| *y,
            &galaxies,
        ));
        let cols = GalacticMap::transform_space_expansion(scale, GalacticMap::find_axis_without_galaxies(
            |(_, x)| *x,
            &galaxies,
        ));

        eprintln!("g {:?}", galaxies);
        eprintln!("r {:?} c {:?}", rows, cols);

        GalacticMap {
            galaxies: galaxies
                .into_iter()
                .map(|(gy, gx)| {
                    let xoff = cols[gx.min(cols.len() - 1)];
                    let yoff = rows[gy.min(rows.len() - 1)];

                    (gy + yoff, gx + xoff)
                })
                .zip(1..)
                .map(|(g, num)| (num, g))
                .collect(),
        }
    }

    fn generate_distances(&self) -> Vec<GalaxyPair> {
        let gnums = 1..=self.galaxies.len();

        gnums.combinations(2).map(|r| self.find_distance(r[0], r[1])).collect()        
    }
}

pub fn day11() {
    //let game_file = read_file_as_text("./inputs/day11test.txt").lines();
    let game_file = read_file_as_text("./inputs/day11real.txt").lines();
    let map = GalacticMap::from_lines(1000000, game_file.into_iter().filter_map(|s| s.ok()));

    println!("Hello {:?}", map);
    println!("Distance {:?}", map.find_distance(5, 9));

    let dists = map.generate_distances();
    println!("{}", dists.into_iter().map(|(_, _, d)| d).sum::<usize>());
}
