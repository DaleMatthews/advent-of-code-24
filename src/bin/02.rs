advent_of_code::solution!(2);
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn get_slopes(input: &[i32]) -> Vec<i32> {
    input
        .iter()
        .tuple_windows()
        .map(|(x, y)| y - x)
        .collect::<Vec<i32>>()
}

fn report_is_unsafe_at_edge(report: &[i32]) -> Option<usize> {
    let slopes = get_slopes(report);
    let increasing = slopes[0] > 0;
    for (i, slope) in slopes.iter().enumerate() {
        if (increasing && *slope < 0)
            || (!increasing && *slope > 0)
            || (slope.abs() > 3 || *slope == 0)
        {
            return Some(i);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    let mut num_safe = 0;
    for report in reports {
        if report_is_unsafe_at_edge(&report).is_none() {
            num_safe += 1;
        }
    }
    Some(num_safe)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    let mut num_safe = 0;
    for report in reports {
        let res = report_is_unsafe_at_edge(&report);
        match res {
            None => num_safe += 1,
            Some(i) => {
                // try removing report[i] or report[i+1] or report[i-1]
                let mut dampened_reports = vec![];
                let mut report2 = report.clone();
                report2.remove(i);
                dampened_reports.push(report2);
                // println!("New vector: {:?}", reports);
                let mut report3 = report.clone();
                report3.remove(i + 1);
                dampened_reports.push(report3);
                if i > 0 {
                    let mut report4 = report.clone();
                    report4.remove(i - 1);
                    dampened_reports.push(report4);
                }
                for r in dampened_reports {
                    if report_is_unsafe_at_edge(&r).is_none() {
                        num_safe += 1;
                        break;
                    }
                }
            }
        }
    }
    Some(num_safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
