advent_of_code::solution!(4);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .split("\n")
        .map(|line| line.chars().collect())
        .collect()
}

fn count_xmas_rows(grid: &[Vec<char>]) -> Option<usize> {
    grid
        .iter()
        .map(|line| {
            let fwd = line.iter().collect::<String>();
            let bwd = line.iter().rev().collect::<String>();
            fwd.matches("XMAS").count() + bwd.matches("XMAS").count()
        })
        .reduce(|acc, x| acc + x)
}

fn count_xmas_cols(grid: &[Vec<char>]) -> usize {
    let mut sum = 0;
    for j in 0..grid[0].len() {
        let mut fwd = String::new();
        for row in grid {
            fwd.push(row[j]);
        }
        let bwd: String = fwd.chars().rev().collect();
        sum += fwd.matches("XMAS").count() + bwd.matches("XMAS").count();
    }
    sum
}

fn count_xmas_diags_downward(grid: &[Vec<char>]) -> usize {
    let mut sum = 0;
    for i in 0..grid.len() {
        let mut fwd = String::new();
        for j in 0..grid[0].len() {
            if i + j < grid.len() {
                fwd.push(grid[i + j][j]);
            }
        }
        let bwd: String = fwd.chars().rev().collect();
        sum += fwd.matches("XMAS").count() + bwd.matches("XMAS").count();
    }
    for j in 1..grid[0].len() {
        let mut fwd = String::new();
        for i in 0..grid.len() {
            if i + j < grid[0].len() {
                fwd.push(grid[i][i + j]);
            }
        }
        let bwd: String = fwd.chars().rev().collect();
        sum += fwd.matches("XMAS").count() + bwd.matches("XMAS").count();
    }
    sum
}

fn count_xmas_diags_upward(grid: &[Vec<char>]) -> usize {
    let mut sum = 0;
    for i in 0..grid.len() as isize {
        let mut fwd = String::new();
        for j in 0..grid[0].len() as isize {
            if i - j >= 0 {
                fwd.push(grid[(i - j) as usize][j as usize]);
            }
        }
        let bwd: String = fwd.chars().rev().collect();
        sum += fwd.matches("XMAS").count() + bwd.matches("XMAS").count();
    }
    for j in 1..grid[0].len() as isize {
        let mut fwd = String::new();
        for i in 0..grid.len() as isize {
            if j + i < grid[0].len() as isize {
                fwd.push(grid[(grid.len() as isize - 1 - i) as usize][(j + i) as usize]);
            }
        }
        let bwd: String = fwd.chars().rev().collect();
        sum += fwd.matches("XMAS").count() + bwd.matches("XMAS").count();
    }
    sum
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    let mut sum = count_xmas_rows(&grid)?;
    sum += count_xmas_cols(&grid);
    sum += count_xmas_diags_downward(&grid);
    sum += count_xmas_diags_upward(&grid);
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    let mut sum: usize = 0;
    for i in 0..grid.len() - 2 {
        for j in 0..grid[0].len() - 2 {
            if grid[i + 1][j + 1] == 'A' 
                && ((grid[i][j] == 'M' && grid[i + 2][j + 2] == 'S')
                    || (grid[i][j] == 'S' && grid[i + 2][j + 2] == 'M'))
                && ((grid[i][j + 2] == 'M' && grid[i + 2][j] == 'S')
                    || (grid[i][j + 2] == 'S' && grid[i + 2][j] == 'M'))
                {
                    sum += 1;
                }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
