use std::collections::{HashMap, VecDeque};

use regex::Regex;

advent_of_code::solution!(10);

const ON: u8 = b'#';

type Joltage = u64;

pub fn part_one(input: &str) -> Option<u64> {
    let result = input
        .lines()
        .map(|line| parse_schematics(line))
        .map(|(goal, steps, joltages)| bfs(goal, steps, joltages))
        .fold(0, |a, n| a + n);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse_schematics(line: &str) -> (u64, Vec<u64>, Vec<Joltage>) {
    let re = Regex::new(r"^\[(?P<goal>[^\]]+)]\s+(?P<steps>(\([^)]*\)\s*)+)\{(?P<joltages>[^}]*)}")
        .unwrap();

    let caps = re.captures(line).unwrap();

    let goal: u64 = caps["goal"]
        .bytes()
        .enumerate()
        .fold(0u64, |d, (i, b)| d | (u64::from(b == ON) << i));

    let re_step = Regex::new(r"\(([^)]*)\)").unwrap();
    let steps: Vec<u64> = re_step
        .captures_iter(&caps["steps"])
        .map(|c| {
            let mut mask: u64 = 0;
            for n in c[1].split(',').map(|n| n.parse::<usize>().unwrap()) {
                mask |= 1 << n;
            }
            mask
        })
        .collect();

    let joltages: Vec<Joltage> = caps["joltages"]
        .split(',')
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    // println!(".{:0len$b}", goal, len = joltages.len());
    (goal, steps, joltages)
}

fn adjs(cur: u64, steps: &Vec<u64>) -> Vec<u64> {
    steps.iter().map(|step| cur ^ step).collect()
}

fn bfs(goal: u64, steps: Vec<u64>, joltages: Vec<Joltage>) -> u64 {
    // let n = joltages.len();
    let mut dist: HashMap<u64, u64> = HashMap::new();
    let mut queue = VecDeque::new();

    dist.insert(0, 0);
    queue.push_back(0);

    while let Some(v) = queue.pop_front() {
        for w in adjs(v, &steps) {
            if !dist.contains_key(&w) {
                let dist_w = dist[&v] + 1;
                // println!(".{:0len$b} -> {:0len$b} at {}", v, w, dist_w, len = n);

                if w == goal {
                    return dist_w;
                }

                dist.insert(w, dist_w);
                queue.push_back(w);
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
