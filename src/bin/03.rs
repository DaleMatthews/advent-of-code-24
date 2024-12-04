advent_of_code::solution!(3);
use regex::Regex;

fn parse_input(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+,\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| {
            let (_, [xy]) = c.extract();

            let parts: Vec<i32> = xy
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();
            parts[0] * parts[1]
        })
        .sum::<i32>()
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(parse_input(input))
}

pub fn part_two(input: &str) -> Option<i32> {
    let re_newlines = Regex::new(r"\r?\n").unwrap();
    let input = re_newlines.replace_all(input, "").to_string();
    let re_disabled: Regex = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    let input = re_disabled.replace_all(&input, "").to_string();
    Some(parse_input(&input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 3));
        assert_eq!(result, Some(8));
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 4));
        assert_eq!(result, Some(8));
    }
}
