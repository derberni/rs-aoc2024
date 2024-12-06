use anyhow::*;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::read_to_string;

fn part1(rules: &HashMap<u32, Vec<u32>>, manuals: &Vec<Vec<u32>>) -> u32 {
    manuals.into_iter().fold(0, |acc, manual| {
        let unordered = (0..manual.len()).any(|i| {
            let page = manual[i];
            match rules.get(&page) {
                None => false,
                Some(page_rules) => manual[0..i].iter().any(|j| page_rules.contains(j)),
            }
        });
        match unordered {
            true => acc,
            false => acc + manual.get(manual.len() / 2).unwrap(),
        }
    })
}

fn part2(rules: &HashMap<u32, Vec<u32>>, manuals: &Vec<Vec<u32>>) -> u32 {
    manuals.into_iter().fold(0, |acc, manual| {
        let unordered = (0..manual.len()).any(|i| {
            let page = manual[i];
            match rules.get(&page) {
                None => false,
                Some(page_rules) => manual[0..i].iter().any(|j| page_rules.contains(j)),
            }
        });
        match unordered {
            true => {
                let mut sorted_manual = manual.clone();
                sorted_manual.sort_by(|a, b| match rules.get(a) {
                    None => Ordering::Equal,
                    Some(page_rules) => match page_rules.contains(b) {
                        true => Ordering::Less,
                        false => Ordering::Equal,
                    },
                });
                acc + sorted_manual.get(sorted_manual.len() / 2).unwrap()
            }
            false => acc,
        }
    })
}

fn parse_rules(rules: &str) -> HashMap<u32, Vec<u32>> {
    let mut rules_map: HashMap<u32, Vec<u32>> = HashMap::new();
    let rule_re = Regex::new(r"(\d+)\|(\d+)").unwrap();
    rules.lines().for_each(|line| {
        let captures = rule_re.captures(line).unwrap();
        let (_, [page, before]) = captures.extract();
        let (page, before) = (page.parse::<u32>().unwrap(), before.parse::<u32>().unwrap());

        match rules_map.entry(page) {
            Entry::Vacant(entry) => {
                entry.insert(vec![before]);
            }
            Entry::Occupied(mut entry) => {
                entry.get_mut().push(before);
            }
        }
    });

    rules_map
}

fn parse_manuals(manuals: &str) -> Vec<Vec<u32>> {
    manuals
        .lines()
        .map(|line| line.split(',').map(|d| d.parse::<u32>().unwrap()).collect())
        .collect()
}

fn main() -> Result<()> {
    let input = read_to_string("input/05.txt")?;
    let (rules, manuals) = input.split_once("\n\n").unwrap();
    let (rules, manuals) = (parse_rules(rules), parse_manuals(manuals));

    let part1_result = part1(&rules, &manuals);
    println!("Part 1 result: {}", part1_result);

    let part2_result = part2(&rules, &manuals);
    println!("Part 2 result: {}", part2_result);

    Ok(())
}

#[test]
fn part1_example() {
    let rules = parse_rules(
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13",
    );
    let manuals = parse_manuals(
        "75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
    );
    assert_eq!(part1(&rules, &manuals), 143);
}

#[test]
fn part2_example() {
    let rules = parse_rules(
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13",
    );
    let manuals = parse_manuals(
        "75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
    );
    assert_eq!(part2(&rules, &manuals), 123);
}
