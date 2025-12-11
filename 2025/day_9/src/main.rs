use itertools::Itertools;
use std;
use std::cmp::max;
use std::cmp::min;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

fn make_poss(input_str: &str) -> Vec<Pos> {
    let mut poss: Vec<Pos> = Vec::new();
    for row_str in input_str.lines() {
        let v: Vec<usize> = row_str
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        poss.push(Pos { x: v[0], y: v[1] })
    }
    poss
}

fn find_area(a: &Pos, b: &Pos) -> usize {
    (max(a.x, b.x) - min(a.x, b.x) + 1) * (max(a.y, b.y) - min(a.y, b.y) + 1)
}

fn part_1(file_name: &str) -> usize {
    let input_str = read_to_string(file_name).expect("Can't open file");
    let poss = make_poss(&input_str);
    let pos_ids: Vec<usize> = (0..poss.len()).collect();
    let mut pairs: Vec<(usize, usize)> = pos_ids
        .into_iter()
        .combinations(2)
        .map(|v| (v[0], v[1]))
        .collect();
    pairs.sort_by_key(|(a, b)| find_area(&poss[*a], &poss[*b]));
    let (a, b) = pairs.last().unwrap();
    find_area(&poss[*a], &poss[*b])
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Disp {
    Colour,
    NoColour,
}

fn make_grid(poss: &Vec<Pos>) -> Vec<Vec<Disp>> {
    #[derive(Debug, Clone, Copy, PartialEq)]
    enum BuildDisp {
        Line,
        Out,
        Unknown,
    }
    let max_x = poss.iter().map(|p| p.x).max().unwrap() + 1;
    let max_y = poss.iter().map(|p| p.y).max().unwrap() + 1;
    let min_x = poss.iter().map(|p| p.x).min().unwrap() + 1;
    let min_y = poss.iter().map(|p| p.y).min().unwrap() + 1;
    println!("min x {:?} min y {:?}", min_x, min_y);
    println!("max x {:?} max y {:?}", max_x, max_y);
    panic!("");
    let mut grid: Vec<Vec<BuildDisp>> = vec![vec![BuildDisp::Unknown; max_x + 1]; max_y + 1];

    let mut prev_pos = &poss[poss.len() - 1];
    for pos in poss.iter() {
        for x in min(prev_pos.x, pos.x)..=max(prev_pos.x, pos.x) {
            for y in min(prev_pos.y, pos.y)..=max(prev_pos.y, pos.y) {
                grid[y][x] = BuildDisp::Line;
            }
        }
        prev_pos = pos;
    }
    println!("About to do heads");

    let mut heads: Vec<Pos> = vec![Pos { x: max_x, y: max_y }];

    while heads.len() > 0 {
        let head = heads.pop().unwrap();
        let y_start = match head.y {
            0 => 0,
            _ => head.y - 1,
        };
        let y_finish = match head.y {
            v if v == max_y => max_y,
            _ => head.y + 1,
        };

        for y in y_start..=y_finish {
            let x_start = match head.x {
                0 => 0,
                _ => head.x - 1,
            };
            let x_finish = match head.x {
                v if v == max_x => max_x,
                _ => head.x + 1,
            };
            for x in x_start..=x_finish {
                if x == head.x && y == head.y {
                    grid[y][x] = BuildDisp::Out;
                } else if grid[y][x] == BuildDisp::Unknown {
                    heads.push(Pos { x: x, y: y });
                }
            }
        }
    }
    let mut colour_grid: Vec<Vec<Disp>> = Vec::new();

    for row in grid.iter() {
        colour_grid.push(
            row.iter()
                .map(|x| match x {
                    BuildDisp::Line => Disp::Colour,
                    BuildDisp::Out => Disp::NoColour,
                    BuildDisp::Unknown => Disp::Colour,
                })
                .collect(),
        );
    }
    colour_grid
}

fn all_colour(poss: &Vec<Pos>, a: usize, b: usize) -> bool {
    let pos_a = poss[a];
    let pos_b = poss[b];

    let min_x = min(pos_a.x, pos_b.x);
    let max_x = max(pos_a.x, pos_b.x);
    let min_y = min(pos_a.y, pos_b.y);
    let max_y = max(pos_a.y, pos_b.y);

    for pos in poss.iter() {
        if pos.x > min_x && pos.x < max_x && pos.y > min_y && pos.y < max_y {
            return false;
        }
    }
    true
}

fn in_polygon(
    verticals: &Vec<(usize, usize, usize)>,
    horizontals: &Vec<(usize, usize, usize)>,
    px: usize,
    py: usize,
) -> bool {
    let mut count = 0;
    for (line_y, line_min_x, line_max_x) in horizontals.iter() {
        if py == *line_y {
            if px >= *line_min_x && px <= *line_max_x {
                return true;
            } else if px > *line_max_x {
                count += 1;
            }
        }
    }
    for (line_x, line_min_y, line_max_y) in verticals.iter() {
        if px == *line_x && py >= *line_min_y && py <= *line_max_y {
            return true;
        }
        if px > *line_x && py > *line_min_y && py < *line_max_y {
            count += 1;
        }
    }
    count % 2 == 1
}
fn find_polygon_lines(poss: &Vec<Pos>) -> (Vec<(usize, usize, usize)>, Vec<(usize, usize, usize)>) {
    let mut verticals: Vec<(usize, usize, usize)> = Vec::new();
    let mut horizontals: Vec<(usize, usize, usize)> = Vec::new();
    let mut prev_pos = &poss[poss.len() - 1];
    for pos in poss.iter() {
        let line_min_x = min(prev_pos.x, pos.x);
        let line_max_x = max(prev_pos.x, pos.x);
        let line_min_y = min(prev_pos.y, pos.y);
        let line_max_y = max(prev_pos.y, pos.y);
        if prev_pos.x == pos.x {
            verticals.push((pos.x, line_min_y, line_max_y));
        } else if prev_pos.y == pos.y {
            horizontals.push((pos.y, line_min_x, line_max_x));
        }
        prev_pos = pos;
    }
    (verticals, horizontals)
}

fn all_in_polygon(
    verticals: &Vec<(usize, usize, usize)>,
    horizontals: &Vec<(usize, usize, usize)>,
    a: &Pos,
    b: &Pos,
) -> bool {
    let min_x = min(a.x, b.x);
    let max_x = max(a.x, b.x);
    let min_y = min(a.y, b.y);
    let max_y = max(a.y, b.y);

    for (line_x, line_min_y, line_max_y) in verticals {
        if *line_x > min_x && *line_x < max_x && *line_min_y < max_y && *line_max_y > min_y {
            return false;
        }
    }
    for (line_y, line_min_x, line_max_x) in horizontals {
        if *line_y > min_y && *line_y < max_y && *line_min_x < max_x && *line_max_x > min_x {
            return false;
        }
    }

    for x in [min_x, max_x] {
        for y in min_y..=max_y {
            if !in_polygon(verticals, horizontals, x, y) {
                return false;
            }
        }
    }
    for y in [min_y, max_y] {
        for x in min_x..=max_x {
            if !in_polygon(verticals, horizontals, x, y) {
                return false;
            }
        }
    }
    true
}

fn first_all_in_polygon(
    poss: &Vec<Pos>,
    verticals: &Vec<(usize, usize, usize)>,
    horizontals: &Vec<(usize, usize, usize)>,
    pairs: &Vec<(usize, usize)>,
) -> (usize, usize) {
    for (a, b) in pairs.iter() {
        if all_in_polygon(verticals, horizontals, &poss[*a], &poss[*b]) {
            return (*a, *b);
        }
    }
    panic!("None all in polygon");
}

fn part_2(file_name: &str) -> usize {
    let input_str = read_to_string(file_name).expect("Can't open file");
    let poss = make_poss(&input_str);
    let pos_ids: Vec<usize> = (0..poss.len()).collect();
    let mut pairs: Vec<(usize, usize)> = pos_ids
        .into_iter()
        .combinations(2)
        .map(|v| (v[0], v[1]))
        .collect();
    pairs.sort_by_key(|(a, b)| find_area(&poss[*a], &poss[*b]));
    pairs.reverse();
    let (verticals, horizontals) = find_polygon_lines(&poss);

    let (a, b) = first_all_in_polygon(&poss, &verticals, &horizontals, &pairs);
    find_area(&poss[a], &poss[b])
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
        assert_eq!(total, 50);
    }
    #[test]
    fn test_find_area() {
        let total = find_area(&Pos { x: 2, y: 5 }, &Pos { x: 11, y: 1 });
        assert_eq!(total, 50);
    }
    #[test]
    fn part_2_example() {
        let total = part_2("example.txt");
        assert_eq!(total, 24);
    }
    #[test]
    fn test_all_in_polygon() {
        let verticals: Vec<(usize, usize, usize)> =
            vec![(7, 1, 3), (11, 1, 7), (9, 5, 7), (2, 3, 5)];
        let horizontals: Vec<(usize, usize, usize)> =
            vec![(1, 7, 11), (7, 9, 11), (5, 2, 9), (3, 2, 7)];
        let in_polygon = all_in_polygon(
            &verticals,
            &horizontals,
            &Pos { x: 9, y: 5 },
            &Pos { x: 2, y: 3 },
        );
        assert_eq!(in_polygon, true);
    }
    #[test]
    fn test_in_polygon() {
        let verticals: Vec<(usize, usize, usize)> =
            vec![(7, 1, 3), (11, 1, 7), (9, 5, 7), (2, 3, 5)];
        let horizontals: Vec<(usize, usize, usize)> =
            vec![(1, 7, 11), (7, 9, 11), (5, 2, 9), (3, 2, 7)];
        let in_polygon = in_polygon(&verticals, &horizontals, 8, 3);
        assert_eq!(in_polygon, true);
    }
}
