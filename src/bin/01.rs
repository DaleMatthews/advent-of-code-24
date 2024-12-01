advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lines = input.lines();
    let mut numbers1 = Vec::new();
    let mut numbers2 = Vec::new();
    for line in lines {
        let mut parts = line.split_whitespace();
        let numbers: (i32, i32) = (
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        );
        numbers1.push(numbers.0);
        numbers2.push(numbers.1);
    }
    numbers1.sort();
    numbers2.sort();
    (numbers1, numbers2)
}

pub fn part_one(input: &str) -> Option<i32> {
    let (numbers1, numbers2) = parse_input(input);
    let mut diff_sum: i32 = 0;
    for i in 0..numbers1.len() {
        diff_sum += (numbers1[i] - numbers2[i]).abs();
    }
    Some(diff_sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (numbers1, numbers2) = parse_input(input);
    let mut similarity: i32 = 0;
    for num in numbers1.iter() {
        let count: i32 = numbers2
            .iter()
            .filter(|&n| n == num)
            .count()
            .try_into()
            .unwrap();
        similarity += num * count;
    }
    Some(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11))
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
