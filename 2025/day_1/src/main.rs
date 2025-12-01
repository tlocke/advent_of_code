use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part_1() {
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

fn next_dial(dial: &i64, is_forward: &bool) -> i64 {
    match (dial, is_forward) {
        (99, true) => 0,
        (0, false) => 99,
        (_, true) => dial + 1,
        (_, false) => dial - 1,
    }
}

fn part_2(file_name: &str) -> i64 {
    let path = Path::new(file_name);
    let file = File::open(&path).expect("Can't find file");
    let reader = io::BufReader::new(file);
    let mut dial: i64 = 50;
    let mut zeros: i64 = 0;
    for line in reader.lines() {
        let line_str = line.unwrap();

        let chars: Vec<char> = line_str.chars().collect();
        let is_forward: bool = match chars.first().unwrap() {
            'R' => true,
            'L' => false,
            _ => panic!("Direct must be 'L' or 'R'."),
        };

        let click_slice = &chars[1..];
        let click_str: String = click_slice.iter().collect();
        let clicks: i64 = click_str.parse().unwrap();

        for _ in 0..clicks {
            let new_dial = next_dial(&dial, &is_forward);
            if new_dial == 0 {
                zeros += 1;
            }
            dial = new_dial;
        }
    }
    zeros
}

fn main() {
    part_1();
    let zeros = part_2("input.txt");
    println!("part 2 {zeros}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2_example() {
        let zeros = part_2("example.txt");
        assert_eq!(zeros, 6);
    }
}
