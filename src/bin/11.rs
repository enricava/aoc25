use std::collections::{HashMap, HashSet};

advent_of_code::solution!(11);

const SOURCE: &str = "you";
const EXIT: &str = "out";

pub fn dfs(cur: &str, adjs: &HashMap<&str, Vec<&str>>, visited: &mut HashSet<String>) -> u64 {
    if cur == EXIT {
        return 1;
    }

    let mut paths = 0;
    visited.insert(cur.to_string());

    for &adj in &adjs[cur] {
        if !visited.contains(adj) {
            paths += dfs(adj, adjs, visited);
        }
    }

    visited.remove(cur);

    paths
}

pub fn part_one(input: &str) -> Option<u64> {
    let adjs = parse_adjs(input);

    let sol = dfs(SOURCE, &adjs, &mut HashSet::new());

    Some(sol)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse_adjs(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut adjs = HashMap::new();

    input.lines().map(parse_line).for_each(|(src, dsts)| {
        adjs.insert(src, dsts);
    });

    adjs
}

fn parse_line(line: &str) -> (&str, Vec<&str>) {
    let mut parts = line.split(": ");

    let src = parts.next().expect("source");
    let dsts = parts.next().expect("adjs");

    (src, dsts.split_whitespace().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
