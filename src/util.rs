use num::Num;
use std::fs::File;
use std::io::BufReader;
use std::ops::Deref;
use std::str::FromStr;

pub fn read_file_as_text(path: &str) -> BufReader<File> {
    let f = File::open(path).unwrap();
    BufReader::new(f)
}

pub fn generate_adjacencies<T, T2: Deref<Target = [T]>>(
    cube: &[T2],
    x: usize,
    y: usize,
) -> Vec<(usize, usize)> {
    let ix = x as isize;
    let iy = y as isize;

    [
        (iy - 1, ix - 1),
        (iy - 1, ix),
        (iy - 1, ix + 1),
        (iy, ix - 1),
        (iy, ix + 1),
        (iy + 1, ix - 1),
        (iy + 1, ix),
        (iy + 1, ix + 1),
    ]
    .into_iter()
    .filter(|&(ay, ax)| {
        ay >= 0 && ax >= 0 && ay < (cube.len() as isize) && ax < (cube[0].len() as isize)
    })
    .map(|(ay, ax)| (ay as usize, ax as usize))
    .collect()
}

pub fn split_numbers_by_space<T: Num + FromStr>(val: &str) -> Vec<T> {
    val.trim()
        .split(' ')
        .filter_map(|s| s.parse::<T>().ok())
        .collect()
}

pub fn split_numbers_by_comma<T: Num + FromStr>(val: &str) -> Vec<T> {
    val.trim()
        .split(',')
        .filter_map(|s| s.parse::<T>().ok())
        .collect()
}
