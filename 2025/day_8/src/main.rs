use itertools::Itertools;
use std;
use std::collections::HashSet;
use std::fs::read_to_string;

use std::ops::Sub;

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, other: Pos) -> Pos {
        Pos {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Pos {
    fn len(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2) + self.z.pow(2)) as f64).sqrt()
    }
    fn distance(self, other: Pos) -> f64 {
        (self - other).len()
    }
}

fn make_poss(input_str: &str) -> Vec<Pos> {
    let mut poss: Vec<Pos> = Vec::new();
    for row_str in input_str.lines() {
        let v: Vec<i64> = row_str
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        poss.push(Pos {
            x: v[0],
            y: v[1],
            z: v[2],
        })
    }
    poss
}

fn find_group(groups: &Vec<HashSet<usize>>, pos_id: &usize) -> usize {
    for (i, group) in groups.iter().enumerate() {
        if group.contains(pos_id) {
            return i;
        }
    }
    panic!("Can't find group");
}

fn groups_equal(groups: &Vec<HashSet<usize>>, a: usize, b: usize) -> bool {
    let group_a = &groups[a];
    let group_b = &groups[b];
    std::ptr::eq(&group_a, &group_b)
}

fn groups_combine(groups: &Vec<HashSet<usize>>, a: usize, b: usize) -> HashSet<usize> {
    let group_a = &groups[a];
    let group_b = &groups[b];
    group_a.union(&group_b).map(|x| *x).collect()
}

fn part_1(file_name: &str, num_wires: usize) -> usize {
    let input_str = read_to_string(file_name).expect("Can't open file");
    let poss = make_poss(&input_str);
    let pos_ids: Vec<usize> = (0..poss.len()).collect();
    println!("pos_ids {:?}", pos_ids);
    let mut groups: Vec<HashSet<usize>> = pos_ids.iter().map(|x| HashSet::from([*x])).collect();
    let mut pairs: Vec<(usize, usize)> = pos_ids
        .into_iter()
        .combinations(2)
        .map(|v| (v[0], v[1]))
        .collect();
    pairs.sort_by(|(a1, a2), (b1, b2)| {
        poss[*a1]
            .distance(poss[*a2])
            .partial_cmp(&poss[*b1].distance(poss[*b2]))
            .unwrap()
    });

    for (pos_a, pos_b) in &pairs[0..num_wires] {
        let group_a_ind = find_group(&groups, &pos_a);
        let group_b_ind = find_group(&groups, &pos_b);
        if !groups_equal(&groups, group_a_ind, group_b_ind) {
            let group = groups_combine(&groups, group_a_ind, group_b_ind);
            groups = groups
                .into_iter()
                .enumerate()
                .filter(|(x, _)| ![group_a_ind, group_b_ind].contains(x))
                .map(|(_, x)| x)
                .collect();
            groups.push(group);
        }
    }
    println!("{:?}", groups);

    let mut sizes: Vec<usize> = groups.into_iter().map(|x| x.len()).collect();
    println!("sizes {:?}", sizes);
    sizes.sort();
    sizes.reverse();
    sizes[0..3].into_iter().product()
}

fn part_2(file_name: &str) -> i64 {
    let input_str = read_to_string(file_name).expect("Can't open file");
    let poss = make_poss(&input_str);
    let pos_ids: Vec<usize> = (0..poss.len()).collect();
    println!("pos_ids {:?}", pos_ids);
    let mut groups: Vec<HashSet<usize>> = pos_ids.iter().map(|x| HashSet::from([*x])).collect();
    let mut pairs: Vec<(usize, usize)> = pos_ids
        .into_iter()
        .combinations(2)
        .map(|v| (v[0], v[1]))
        .collect();
    pairs.sort_by(|(a1, a2), (b1, b2)| {
        poss[*a1]
            .distance(poss[*a2])
            .partial_cmp(&poss[*b1].distance(poss[*b2]))
            .unwrap()
    });

    for (pos_a, pos_b) in pairs {
        let group_a_ind = find_group(&groups, &pos_a);
        let group_b_ind = find_group(&groups, &pos_b);
        if !groups_equal(&groups, group_a_ind, group_b_ind) {
            let group = groups_combine(&groups, group_a_ind, group_b_ind);
            groups = groups
                .into_iter()
                .enumerate()
                .filter(|(x, _)| ![group_a_ind, group_b_ind].contains(x))
                .map(|(_, x)| x)
                .collect();
            groups.push(group);
        }
        if groups.len() == 1 {
            return poss[pos_a].x * poss[pos_b].x;
        }
    }
    panic!("Groups not 1")
}

fn main() {
    let total_1 = part_1("input.txt", 1000);
    println!("{total_1}");
    let total_2 = part_2("input.txt");
    println!("{total_2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let total = part_1("example.txt", 10);
        assert_eq!(total, 40);
    }
    #[test]
    fn part_2_example() {
        let total = part_2("example.txt");
        assert_eq!(total, 25272);
    }
}
