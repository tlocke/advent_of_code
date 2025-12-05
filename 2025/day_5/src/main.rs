use std::cmp::max;
use std::fs;

fn part_1(file_name: &str) -> i64 {
    let input_str = fs::read_to_string(file_name).expect("Can't open file");
    let mut parse_range = true;
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut total = 0;
    for row_str in input_str.lines() {
        if row_str.len() == 0 {
            parse_range = false;
            continue;
        }
        if parse_range {
            let range: Vec<i64> = row_str.split('-').map(|x| x.parse().unwrap()).collect();
            ranges.push((range[0], range[1]))
        } else {
            let id: i64 = row_str.parse().unwrap();
            if ranges
                .iter()
                .any(|(start, finish)| id >= *start && id <= *finish)
            {
                total += 1;
            }
        }
    }
    total
}

fn part_2_from_str(input_str: &str) -> i64 {
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    for row_str in input_str.lines() {
        if row_str.len() == 0 {
            break;
        }
        let range: Vec<i64> = row_str.split('-').map(|x| x.parse().unwrap()).collect();
        ranges.push((range[0], range[1]))
    }
    ranges.sort_by_key(|&(a, _)| a);
    let mut i = 0;
    while i < ranges.len() - 1 {
        if ranges[i].1 >= ranges[i + 1].0 {
            ranges[i] = (ranges[i].0, max(ranges[i].1, ranges[i + 1].1));
            ranges.remove(i + 1);
        } else {
            i += 1;
        }
    }
    ranges
        .iter()
        .map(|(start, finish)| finish - start + 1)
        .sum()
}

fn part_2(file_name: &str) -> i64 {
    let input_str = fs::read_to_string(file_name).expect("Can't open file");
    part_2_from_str(&input_str)
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
        assert_eq!(total, 3);
    }

    #[test]
    fn part_2_example() {
        let total = part_2("example.txt");
        assert_eq!(total, 14);
    }

    #[test]
    fn test_part_2_from_str() {
        let total = part_2_from_str("0-0");
        assert_eq!(total, 1);

        let inp = "0-1\n0-0";
        assert_eq!(part_2_from_str(inp), 2);
    }
}
