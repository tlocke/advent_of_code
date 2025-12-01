use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;

fn main() {
    let path = Path::new("input.txt");
    let file = File::open(&path).expect("Can't find file");
    let reader = io::BufReader::new(file);
    let mut dial: i64 = 50;
    let mut zeros: i64 = 0;
    for line in reader.lines() {
        let line_str = line.unwrap();

        let chars: Vec<char> = line_str.chars().collect();
        let dir: i64 = match chars.first().unwrap() {
            'L' => -1,
            'R' => 1,
            _ => panic!("Direct must be 'L' or 'R'."),
        };

        let step_slice = &chars[1..];
        let step_str: String = step_slice.iter().collect();
        let steps: i64 = step_str.parse().unwrap();
        println!("{dial} {dir} {steps}");

        dial = (dial + (dir * steps)).rem_euclid(100);
        if dial == 0 {
            zeros += 1;
        }
        println!("zeros {zeros}");

    }

}
