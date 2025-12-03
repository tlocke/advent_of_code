use std::fs;

fn part_1(file_name: &str) -> i64 {
    let banks_str = fs::read_to_string(file_name).expect("Can't open file");
    let mut total: i64 = 0;
    for bank_str in banks_str.lines() {
        let mut highest: i64 = 0;
        let chars: Vec<char> = bank_str.chars().collect();

        for i in 0..(chars.len() - 1) {
            for j in (i + 1)..chars.len() {
                let s = format!("{}{}", chars[i], chars[j]);
                highest = std::cmp::max(highest, s.parse().unwrap());
            }
        }
        total += highest;
        println!("{bank_str}");
    }
    total
}

fn main() {
    let total_1 = part_1("input.txt");
    println!("{total_1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let total = part_1("example.txt");
        assert_eq!(total, 357);
    }
}
