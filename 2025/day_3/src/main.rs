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

fn ind_highest(chars: &Vec<char>, start: &usize, finish: &usize) -> usize {
    let mut highest: usize = *start;

    for i in (start + 1)..*finish {
        if chars[i].to_digit(10) > chars[highest].to_digit(10) {
            highest = i
        }
    }
    highest
}

fn bank_value(bank_str: &str) -> i64 {
    let chars: Vec<char> = bank_str.chars().collect();

    let mut start = 0;
    let mut s = String::new();

    for i in (0..12).rev() {
        // println!("{i}");
        let finish = chars.len() - i;
        let ind = ind_highest(&chars, &start, &finish);
        s.push(chars[ind]);
        start = ind + 1;
    }
    s.parse::<i64>().unwrap()
}

fn part_2(file_name: &str) -> i64 {
    let banks_str = fs::read_to_string(file_name).expect("Can't open file");
    let mut total: i64 = 0;
    for bank_str in banks_str.lines() {
        total += bank_value(bank_str);
    }
    total
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
        assert_eq!(total, 357);
    }
    #[test]
    fn part_2_example() {
        let total = part_2("example.txt");
        assert_eq!(total, 3121910778619);
    }

    #[test]
    fn test_bank_value() {
        assert_eq!(bank_value("987654321111111"), 987654321111);
    }
}
