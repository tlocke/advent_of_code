use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
enum CellType {
    SPACE,
    SPLITTER,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum StateType {
    EMPTY,
    BEAM,
}

fn next_state(state: &Vec<StateType>, row: &Vec<CellType>) -> (Vec<StateType>, i64) {
    let mut st: Vec<StateType> = vec![StateType::EMPTY; state.len()];
    let mut splits = 0;
    for (i, (state_type, cell_type)) in state.iter().zip(row.iter()).enumerate() {
        if *state_type == StateType::BEAM {
            match cell_type {
                CellType::SPACE => st[i] = StateType::BEAM,
                CellType::SPLITTER => {
                    st[i - 1] = StateType::BEAM;
                    st[i + 1] = StateType::BEAM;
                    splits += 1;
                }
            }
        }
    }
    (st, splits)
}

fn part_1(file_name: &str) -> i64 {
    let input_str = read_to_string(file_name).expect("Can't open file");
    let mut lines = input_str.lines();
    let mut state: Vec<StateType> = lines
        .next()
        .unwrap()
        .chars()
        .map(|x| match x {
            '.' => StateType::EMPTY,
            'S' => StateType::BEAM,
            _ => panic!("Unknown sign"),
        })
        .collect();
    let mut total = 0;
    for row_str in lines {
        let row: Vec<CellType> = row_str
            .chars()
            .map(|x| match x {
                '.' => CellType::SPACE,
                '^' => CellType::SPLITTER,
                _ => panic!("Unknown cell type"),
            })
            .collect();
        let (n_state, splits) = next_state(&state, &row);

        state = n_state;
        total += splits;
    }
    total
}

fn make_state(length: usize, i: usize) -> Vec<StateType> {
    let mut st: Vec<StateType> = vec![StateType::EMPTY; length];
    st[i] = StateType::BEAM;
    st
}

fn next_states(state: &Vec<StateType>, row: &Vec<CellType>) -> Vec<Vec<StateType>> {
    let mut states: Vec<Vec<StateType>> = Vec::new();
    for (i, (state_type, cell_type)) in state.iter().zip(row.iter()).enumerate() {
        if *state_type == StateType::BEAM {
            match cell_type {
                CellType::SPACE => {
                    states.push(make_state(state.len(), i));
                }
                CellType::SPLITTER => {
                    for j in [i - 1, i + 1] {
                        states.push(make_state(state.len(), j));
                    }
                }
            }
        }
    }
    states
}

fn part_2(file_name: &str) -> i64 {
    let input_str = read_to_string(file_name).expect("Can't open file");
    let mut lines = input_str.lines();
    let state: Vec<StateType> = lines
        .next()
        .unwrap()
        .chars()
        .map(|x| match x {
            '.' => StateType::EMPTY,
            'S' => StateType::BEAM,
            _ => panic!("Unknown sign"),
        })
        .collect();
    let mut states: HashMap<Vec<StateType>, i64> = HashMap::new();
    states.insert(state, 1);

    for row_str in lines {
        println!("{:?}", row_str);
        let row: Vec<CellType> = row_str
            .chars()
            .map(|x| match x {
                '.' => CellType::SPACE,
                '^' => CellType::SPLITTER,
                _ => panic!("Unknown cell type"),
            })
            .collect();
        let mut new_states: HashMap<Vec<StateType>, i64> = HashMap::new();
        for (state, count) in states {
            for st in next_states(&state, &row) {
                let num = if new_states.contains_key(&st) {
                    new_states[&st] + count
                } else {
                    count
                };
                new_states.insert(st, num);
            }
        }

        states = new_states;
        println!("{:?}", states);
    }
    states.values().sum()
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
        assert_eq!(total, 21);
    }
    #[test]
    fn part_2_example() {
        let total = part_2("example.txt");
        assert_eq!(total, 40);
    }
}
