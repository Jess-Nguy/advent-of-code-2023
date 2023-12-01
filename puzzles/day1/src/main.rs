mod part_1;

use crate::part_1::calibration_values_sum;
use std::fs::File;
use std::io::Read;

fn main() {
    let calibration_lines = read_file("./puzzle-input.txt");
    // let calibration_lines = read_file("./calibration-doc-example.txt");

    let calibration_values_sum = calibration_values_sum(calibration_lines.clone());
    println!("Calibration sum: {:?}", calibration_values_sum);
}

// Read a text file and return a vector of strings
fn read_file(filename: &str) -> Vec<String> {
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    contents.lines().map(|s| s.to_string()).collect()
}
