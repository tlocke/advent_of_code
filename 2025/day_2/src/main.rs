use std::collections::HashSet;
use std::fs;

fn part_1(file_name: &str) -> i64 {
    let ranges_str = fs::read_to_string(file_name).expect("Can't open file");
    println!("{ranges_str}");
    let mut reps = 0;
    for pair_str in ranges_str.split(',') {
        println!("{pair_str}");
        let range_vec: Vec<i64> = pair_str
            .split('-')
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect();
        for i in range_vec[0]..=range_vec[1] {
            let s = i.to_string();
            let half = s.len() / 2;
            if s.len() % 2 == 0 && s[..half] == s[half..] {
                reps += i;
                println!("found {s}");
            }
        }
    }
    reps
}

fn has_repeat(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();

    for i in 1..=(s.len() / 2) {
        let chunks = chars.chunks_exact(i);
        if chunks.remainder().is_empty() {
            let set: HashSet<_> = chunks.collect();
            if set.len() == 1 {
                return true;
            }
        }
    }
    false
}

fn part_2(file_name: &str) -> i64 {
    let ranges_str = fs::read_to_string(file_name).expect("Can't open file");
    println!("{ranges_str}");
    let mut reps = 0;
    for pair_str in ranges_str.split(',') {
        println!("{pair_str}");
        let range_vec: Vec<i64> = pair_str
            .split('-')
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect();
        for i in range_vec[0]..=range_vec[1] {
            let s = i.to_string();
            if has_repeat(&s) {
                reps += i;
            }
        }
    }
    reps
}
fn main() {
    let total_1 = part_1("input.txt");
    println!("{total_1}");
    let total_2 = part_2("input.txt");
    println!("{total_2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let total = part_1("example.txt");
        assert_eq!(total, 1227775554);
    }

    #[test]
    fn part_2_example() {
        let total = part_2("example.txt");
        assert_eq!(total, 4174379265);
    }

    #[test]
    fn test_has_repeat() {
        assert_eq!(has_repeat("11"), true);
    }
}
