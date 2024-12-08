use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(5);

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let mut rules: Vec<(usize, usize)> = Vec::new();
    let mut updates: Vec<Vec<usize>> = Vec::new();
    let mut parse_rule = true;
    for line in input.lines() {
        if line.is_empty() {
            parse_rule = false;
            continue;
        }
        if parse_rule {
            let parts: Vec<usize> = line
                .split("|")
                .map(|s| s.parse().unwrap())
                .collect();
            rules.push((parts[0], parts[1]));
        } else {
            let update: Vec<usize> = line
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();
            updates.push(update);
        }
    }
    (rules, updates)
}

struct Rule {
    pub before: HashSet<usize>,
    pub after: HashSet<usize>,
}

fn get_rule_dict(rules: &[(usize, usize)]) -> HashMap::<usize, Rule> {
    let mut rule_dict = HashMap::<usize, Rule>::new();

    for (before, after) in rules.iter() {
        if rule_dict.contains_key(before) {
            let rule = rule_dict.get_mut(before).unwrap();
            if !rule.after.iter().contains(after) {
                rule.after.insert(*after);
            }
        } else {
            rule_dict.insert(
                *before,
                Rule {
                    before: HashSet::new(),
                    after: HashSet::from_iter(vec![*after]),
                },
            );
        }
        if rule_dict.contains_key(after) {
            let rule = rule_dict.get_mut(after).unwrap();
            if !rule.before.iter().contains(before) {
                rule.before.insert(*before);
            }
        } else {
            rule_dict.insert(
                *after,
                Rule {
                    before: HashSet::from_iter(vec![*before]),
                    after: HashSet::new(),
                },
            );
        }
    }
    
    for (value, rule) in rule_dict.iter() {
        println!("{}: {:?} | {:?}", value, rule.before, rule.after);
    }
    rule_dict
}

fn is_valid_update(rule_dict: &HashMap::<usize, Rule>, update: &[usize]) -> bool {
    let mut pages_so_far = HashSet::<&usize>::new();
    let len = update.len();
    for (i, page) in update[..len - 1].iter().enumerate() {
        let rule = rule_dict.get(page).unwrap();
        let after_pages: HashSet<usize> = update[i + 1..].iter().cloned().collect();
        if !rule.before.is_disjoint(&after_pages) {
            return false;
        }
        pages_so_far.insert(page);
    }
    true
}

fn kahn_sort(rule_dict: &HashMap::<usize, Rule>, update: &[usize]) -> Vec<usize> {
    println!("Update: {:?}", update);
    let update_dict = HashSet::<usize>::from_iter(update.iter().cloned());
    let mut sorted = Vec::<usize>::new();
    let mut visited = HashSet::<usize>::new();
    let mut queue = Vec::<usize>::new();
    for page in update.iter() {
        if rule_dict.get(page).unwrap().before.is_disjoint(&update_dict) {
            queue.push(*page);
        }
    }
    while let Some(page) = queue.pop() {
        sorted.push(page);
        visited.insert(page);
        for value in  rule_dict.get(&page).unwrap().after.iter() {
            let rule = rule_dict.get(value).unwrap();
            if rule.before.iter().all(|before| visited.contains(before) || !update_dict.contains(before))
               && update_dict.contains(value) {
                queue.push(*value);
            }
        }
    }
    println!("Sorted: {:?}", sorted);
    sorted
}

pub fn part_one(input: &str) -> Option<usize> {
    let (rules, updates) = parse_input(input);
    let rule_dict = get_rule_dict(&rules);
    let valid_updates: Vec<_> = updates
        .iter()
        .filter(|update| is_valid_update(&rule_dict, update))
        .collect();
    let middles: Vec<_> = valid_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .collect();
    for update in valid_updates.iter() {
        println!("  {:?}", update);
    }
    Some(middles.iter().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (rules, updates) = parse_input(input);
    let rule_dict = get_rule_dict(&rules);
    let valid_updates: Vec<_> = updates
        .iter()
        .filter(|update| !is_valid_update(&rule_dict, update))
        .map(|update| kahn_sort(&rule_dict, update))
        .collect();
    let middles: Vec<_> = valid_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .collect();
    for update in valid_updates.iter() {
        println!("  {:?}", update);
    }
    Some(middles.iter().sum())
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_rule_dict_1() {
        let rules = vec![(1, 2), (2, 3), (3, 4)];
        let res = get_rule_dict(&rules);
        assert_eq!(res.get(&1).unwrap().before, HashSet::from_iter(vec!()));
        assert_eq!(res.get(&1).unwrap().after, HashSet::from_iter(vec!(2)));
        assert_eq!(res.get(&2).unwrap().before, HashSet::from_iter(vec!(1)));
        assert_eq!(res.get(&2).unwrap().after, HashSet::from_iter(vec!(3)));
        assert_eq!(res.get(&3).unwrap().before, HashSet::from_iter(vec!(2)));
        assert_eq!(res.get(&3).unwrap().after, HashSet::from_iter(vec!(4)));
        assert_eq!(res.get(&4).unwrap().before, HashSet::from_iter(vec!(3)));
        assert_eq!(res.get(&4).unwrap().after, HashSet::from_iter(vec!()));
    }

    #[test]
    fn test_rule_dict_2() {
        let rules = vec![(1, 2), (1, 3), (2, 3)];
        let res = get_rule_dict(&rules);
        assert_eq!(res.get(&1).unwrap().before, HashSet::from_iter(vec!()));
        assert_eq!(res.get(&1).unwrap().after, HashSet::from_iter(vec!(2, 3)));
        assert_eq!(res.get(&2).unwrap().before, HashSet::from_iter(vec!(1)));
        assert_eq!(res.get(&2).unwrap().after, HashSet::from_iter(vec!(3)));
        assert_eq!(res.get(&3).unwrap().before, HashSet::from_iter(vec!(1, 2)));
        assert_eq!(res.get(&3).unwrap().after, HashSet::from_iter(vec!()));
    }

    #[test]
    fn test_rule_dict_3() {
        let rules = vec![(1, 2), (2, 3), (1, 3)];
        let res = get_rule_dict(&rules);
        assert_eq!(res.get(&1).unwrap().before, HashSet::from_iter(vec!()));
        assert_eq!(res.get(&1).unwrap().after, HashSet::from_iter(vec!(2, 3)));
        assert_eq!(res.get(&2).unwrap().before, HashSet::from_iter(vec!(1)));
        assert_eq!(res.get(&2).unwrap().after, HashSet::from_iter(vec!(3)));
        assert_eq!(res.get(&3).unwrap().before, HashSet::from_iter(vec!(2, 1)));
        assert_eq!(res.get(&3).unwrap().after, HashSet::from_iter(vec!()));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
