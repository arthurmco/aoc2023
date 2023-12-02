use std::fs::File;
use std::io::BufReader;

pub fn read_file_as_text(path: &str) -> BufReader<File> {
    let f = File::open(path).unwrap();
    BufReader::new(f)
}
