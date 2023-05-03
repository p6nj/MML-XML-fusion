use std::fs::read_to_string;

pub fn read(filename: &str) -> String {
    read_to_string(filename).expect("Reading file")
}
