use std::fs;

/*
            let chars: Vec<char> = s.chars().collect();

            let chunks = chars.chunks_exact(len);
            if chunks.remainder().is_empty() {
                let set: HashSet<_> = chunks.collect();
                if set.len() == 1 {
                    let substr: String = (*set.iter().next().unwrap()).iter().collect();
                    println!("{substr}");
                    reps += substr.parse::<i32>().unwrap();
                }
            }
*/
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
fn main() {
    let total = part_1("input.txt");
    println!("{total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let total = part_1("example.txt");
        assert_eq!(total, 1227775554);
    }
}
