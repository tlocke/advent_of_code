use std::fs;

fn make_store(store_str: &str) -> Vec<Vec<bool>> {
    let mut store: Vec<Vec<bool>> = Vec::new();
    for row_str in store_str.lines() {
        let row: Vec<bool> = row_str.chars().map(|x| x == '@').collect();
        store.push(row)
    }
    store
}

fn adj_occ(store: &Vec<Vec<bool>>, i: &usize, j: &usize) -> i32 {
    let mut count: i32 = 0;
    let row_start = if *i == 0 { 0 } else { i - 1 };
    let max_y = store.len() - 1;
    let row_finish = if *i == max_y { max_y } else { i + 1 };
    for y in row_start..=row_finish {
        let row = &store[y];
        let cell_start = if *j == 0 { 0 } else { j - 1 };
        let max_x = row.len() - 1;
        let cell_finish = if *j == max_x { max_x } else { j + 1 };
        for x in cell_start..=cell_finish {
            if (y, x) != (*i, *j) && row[x] {
                count += 1;
            }
        }
    }
    count
}

fn part_1(file_name: &str) -> i64 {
    let store_str = fs::read_to_string(file_name).expect("Can't open file");
    println!("{store_str}");
    let store = make_store(&store_str);
    let mut total: i64 = 0;

    for i in 0..store.len() {
        for j in 0..store[i].len() {
            if store[i][j] && adj_occ(&store, &i, &j) < 4 {
                total += 1;
            }
        }
    }

    total
}

fn remove(store: &mut Vec<Vec<bool>>) -> bool {
    for i in 0..store.len() {
        for j in 0..store[i].len() {
            if store[i][j] && adj_occ(&store, &i, &j) < 4 {
                store[i][j] = false;
                return true;
            }
        }
    }
    false
}

fn part_2(file_name: &str) -> i64 {
    let store_str = fs::read_to_string(file_name).expect("Can't open file");
    println!("{store_str}");
    let mut store = make_store(&store_str);
    let mut total: i64 = 0;
    while remove(&mut store) {
        total += 1;
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
        assert_eq!(total, 13);
    }

    #[test]
    fn part_2_example() {
        let total = part_2("example.txt");
        assert_eq!(total, 43);
    }

    #[test]
    fn test_adj_occ() {
        let store: Vec<Vec<bool>> = vec![
            vec![false, false, true, true, false],
            vec![true, true, true, false, true],
        ];
        assert_eq!(adj_occ(&store, &0, &3), 3);
    }
}
