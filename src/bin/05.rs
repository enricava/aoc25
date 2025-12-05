use std::cmp::Ordering::{self, Equal, Greater, Less};
use std::cmp::{Reverse, max};
use std::collections::BinaryHeap;

advent_of_code::solution!(5);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Range<T>
where
    T: Ord,
{
    start: T,
    end: T,
}

fn parse_range(range_str: &str) -> Range<u64> {
    let mut range = range_str.split('-');

    let start = range
        .next()
        .expect("has start of range")
        .parse()
        .expect("is a number");

    let end = range
        .next()
        .expect("has end of range")
        .parse()
        .expect("is a number");

    Range { start, end }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sol = 0;

    let mut lines = input.lines();
    let mut sorted_ranges = BinaryHeap::new();

    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        sorted_ranges.push(parse_range(line));
    }

    let mut clean_ranges = Vec::new();

    let sorted = sorted_ranges.into_sorted_vec();
    let mut cur = sorted[0].clone();

    sorted.iter().for_each(|next| {
        if next.start <= cur.end {
            cur.end = max(next.end, cur.end);
        } else {
            clean_ranges.push(cur.clone());

            cur = next.clone();
        }
    });

    if *clean_ranges.last().unwrap() != cur {
        clean_ranges.push(cur);
    }

    for line in &mut lines {
        let num: u64 = line.parse().expect("number");

        let found = clean_ranges
            .binary_search_by(|range| {
                let start_cmp = range.start.cmp(&num);
                let end_cmp = range.end.cmp(&num);

                match (start_cmp, end_cmp) {
                    (Less, Greater) => Equal,
                    (Equal, _) => Equal,
                    (_, Equal) => Equal,
                    (Less, _) => Less,
                    (Greater, _) => Greater,
                }
            })
            .is_ok();

        if found {
            sol += 1;
        }
    }

    Some(sol)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sol = 0;

    let mut lines = input.lines();
    let mut sorted_ranges = BinaryHeap::new();

    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        sorted_ranges.push(parse_range(line));
    }

    let mut clean_ranges = Vec::new();

    let sorted = sorted_ranges.into_sorted_vec();
    let mut cur = sorted[0].clone();

    sorted.iter().for_each(|next| {
        if next.start <= cur.end {
            cur.end = max(next.end, cur.end);
        } else {
            clean_ranges.push(cur.clone());

            cur = next.clone();
        }
    });

    if *clean_ranges.last().unwrap() != cur {
        clean_ranges.push(cur);
    }

    for range in clean_ranges {
        sol += range.end - range.start + 1;
    }

    Some(sol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
