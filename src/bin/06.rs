use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(Clone)]
struct Node {
    obstructed: bool,
    visited: bool,
    obstructed_dirs: HashSet<(isize, isize)>, // directions that this obstruction has been walked into
}

fn parse_input(input: &str) -> (isize, isize, Vec<Vec<Node>>) {
    let mut pos_i = 0;
    let mut pos_j = 0;
    let mut found = false;
    let map: Vec<_> = input
        .trim()
        .lines()
        .map(|line| {
            if !found {
                if let Some(pos) = line.find('^') {
                    pos_j = pos as isize;
                    found = true;
                }
                if !found {
                    pos_i += 1;
                }
            }
            line.chars()
                .map(|c| Node {
                    obstructed: c == '#',
                    visited: false,
                    obstructed_dirs: HashSet::new(),
                })
                .collect()
        })
        .collect();
    (pos_i, pos_j, map)
}

fn _get_dir_char(dir: (isize, isize)) -> char {
    match dir {
        (-1, 0) => '^',
        (1, 0) => 'v',
        (0, -1) => '<',
        (0, 1) => '>',
        _ => panic!("Invalid direction"),
    }
}

fn _print_lab(
    lab: &[Vec<Node>],
    (pos_i, pos_j): (isize, isize),
    (dir_i, dir_j): (isize, isize),
    (new_obstruction_i, new_obstruction_j): (isize, isize),
) {
    for (i, row) in lab.iter().enumerate() {
        for (j, node) in row.iter().enumerate() {
            if i == 6 && j == 4 {
                print!(" "); // starting point
            } else if i == new_obstruction_i as usize && j == new_obstruction_j as usize {
                print!("O");
            } else if node.obstructed {
                print!("#");
            } else if i == pos_i as usize && j == pos_j as usize {
                print!("{}", _get_dir_char((dir_i, dir_j)));
            }else if node.visited {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn turn_right(dir: (isize, isize)) -> (isize, isize) {
    match dir {
        (-1, 0) => (0, 1), // N --> E
        (0, 1) => (1, 0), // E --> S
        (1, 0) => (0, -1), // S --> W
        (0, -1) => (-1, 0), // W --> N
        _ => panic!("Invalid direction"),
    }
}

fn walk(
    lab: &mut Vec<Vec<Node>>,
    (pos_i, pos_j): (isize, isize),
    (dir_i, dir_j): (isize, isize),
) {
    // println!("Walking dir ({}, {})", dir_i, dir_j);
    let len = lab.len() as isize;
    let mut pos_i = pos_i;
    let mut pos_j = pos_j;
    loop {
        // print_lab(lab, (pos_i, pos_j), (dir_i, dir_j));
        let node = &mut lab[pos_i as usize][pos_j as usize];
        node.visited = true;
        if pos_i + dir_i < 0 || pos_i + dir_i >= len || pos_j + dir_j < 0 || pos_j + dir_j >= len {
            println!("Exiting lab @ ({}, {}) via ({}, {})", pos_i, pos_j, dir_i, dir_j);
            break;
        }
        let node = &mut lab[(pos_i + dir_i) as usize][(pos_j + dir_j) as usize];
        if node.obstructed {
            let new_dir = turn_right((dir_i, dir_j));
            // node.obstructed_dirs.insert((dir_i, dir_j));
            walk(lab, (pos_i, pos_j), new_dir);
            break;
        }
        pos_i += dir_i;
        pos_j += dir_j;
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (pos_i, pos_j, mut lab) = parse_input(input);
    walk(&mut lab, (pos_i, pos_j), (-1, 0));
    Some(
        lab.iter()
            .map(|row| row.iter().filter(|node| node.visited).count())
            .sum()
    )
}

fn find_loops(
    lab: &mut Vec<Vec<Node>>,
    (pos_i, pos_j): (isize, isize),
    (dir_i, dir_j): (isize, isize),
    original_lab: bool,
) -> usize {
    let len = lab.len() as isize;
    let mut pos_i = pos_i;
    let mut pos_j = pos_j;
    let mut loops_found = 0;
    loop {
        let node = &mut lab[pos_i as usize][pos_j as usize];
        node.visited = true;
        if pos_i + dir_i < 0 || pos_i + dir_i >= len || pos_j + dir_j < 0 || pos_j + dir_j >= len {
            if original_lab {
                return loops_found;
            }
            return 0; // exiting lab, no loop
        }
        let node = &mut lab[(pos_i + dir_i) as usize][(pos_j + dir_j) as usize];
        if node.obstructed {
            if !original_lab && node.obstructed_dirs.contains(&(dir_i, dir_j)) {
                return 1; // loop found, return 1
            } else if !original_lab {
                node.obstructed_dirs.insert((dir_i, dir_j));
                return find_loops(lab, (pos_i, pos_j), turn_right((dir_i, dir_j)), false);
            } else {
                return loops_found + find_loops(lab, (pos_i, pos_j), turn_right((dir_i, dir_j)), true);
            }
        }
        if original_lab {
            // spawn a new lab with the obstruction that turns right but continue moving forward as well
            let mut new_lab = lab.clone();
            new_lab[(pos_i + dir_i) as usize][(pos_j + dir_j) as usize].obstructed = true;
            new_lab[(pos_i + dir_i) as usize][(pos_j + dir_j) as usize].obstructed_dirs.insert((dir_i, dir_j));
            let found = find_loops(&mut new_lab, (pos_i, pos_j), turn_right((dir_i, dir_j)), false);
            if found == 1 {
                // println!("Found loop!");
                // _print_lab(lab, (pos_i, pos_j), (dir_i, dir_j), (pos_i + dir_i, pos_j + dir_j));
            }
            loops_found += found;
        }
        pos_i += dir_i;
        pos_j += dir_j;
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    // todo, still having an edge-case issue, output is 2138 but should be 1928
    let (pos_i, pos_j, mut lab) = parse_input(input);
    println!("{:?}, {:?}", pos_i, pos_j);
    walk(&mut lab, (pos_i, pos_j), (-1, 0));
    // _print_lab(&lab, (1000, 1000), (-1, 0), (1000, 1000));
    Some(find_loops(&mut lab, (pos_i, pos_j), (-1, 0), true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(6));
    }
}
