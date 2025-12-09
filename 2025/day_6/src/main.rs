use std::fs::read_to_string;

#[derive(Debug)]
enum Operator {
    ADD,
    MULTIPLY,
}

fn part_1(file_name: &str) -> i64 {
    let input_str = read_to_string(file_name).expect("Can't open file");
    let mut lines = input_str.lines().rev();
    let signs: Vec<Operator> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| match x {
            "+" => Operator::ADD,
            "*" => Operator::MULTIPLY,
            _ => panic!("Unknown sign"),
        })
        .collect();
    let mut result: Vec<i64> = signs
        .iter()
        .map(|x| match x {
            Operator::ADD => 0,
            Operator::MULTIPLY => 1,
        })
        .collect();
    for row_str in lines {
        for (i, val) in row_str
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .enumerate()
        {
            result[i] = match signs[i] {
                Operator::ADD => result[i] + val,
                Operator::MULTIPLY => result[i] * val,
            };
        }
    }
    result.iter().sum()
}

fn make_data(input_str: &str) -> (Vec<(Operator, usize)>, Vec<Vec<Vec<char>>>) {
    let mut lines = input_str.lines().rev();
    let mut signs: Vec<Operator> = Vec::new();
    let mut digit_counts: Vec<usize> = Vec::new();
    let mut rows: Vec<Vec<Vec<char>>> = Vec::new();
    for c in lines.next().unwrap().chars() {
        match c {
            '+' => {
                signs.push(Operator::ADD);
                digit_counts.push(0);
            }
            '*' => {
                signs.push(Operator::MULTIPLY);
                digit_counts.push(0);
            }
            ' ' => {
                let last_ind = digit_counts.len() - 1;
                digit_counts[last_ind] += 1;
            }
            _ => panic!("Unknown character"),
        }
    }
    let last_ind = digit_counts.len() - 1;
    digit_counts[last_ind] += 1;
    for line in lines.rev() {
        let mut row = Vec::new();
        let mut start = 0;
        for digit_count in &digit_counts {
            let finish = start + digit_count;
            let digits: Vec<char> = line[start..finish].chars().collect();
            row.push(digits);
            start = finish + 1;
        }
        rows.push(row);
    }
    println!("{:?}", rows);
    println!("{:?}", digit_counts);
    (
        signs.into_iter().zip(digit_counts.into_iter()).collect(),
        rows,
    )
}

fn part_2(file_name: &str) -> i64 {
    let input_str = read_to_string(file_name).expect("Can't open file");
    let (columns, rows) = make_data(&input_str);
    let mut total: i64 = 0;
    for (i, (operator, digits)) in columns.iter().enumerate() {
        println!("{:?}", digits);
        let mut vals: Vec<i64> = Vec::new();
        for j in 0..*digits {
            let mut s: String = String::new();
            for row in &rows {
                s.push(row[i][j]);
            }
            println!("{:?}", s);
            vals.push(s.trim().parse().unwrap());
        }
        total += match operator {
            Operator::ADD => vals.into_iter().sum::<i64>(),
            Operator::MULTIPLY => vals.into_iter().product(),
        }
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
        assert_eq!(total, 4277556);
    }
    #[test]
    fn part_2_example() {
        let total = part_2("example.txt");
        assert_eq!(total, 3263827);
    }
}
