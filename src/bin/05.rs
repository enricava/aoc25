use std::cmp::Ordering::{Equal, Greater, Less};
use std::cmp::max;
use std::collections::BinaryHeap;
use std::str::Lines;

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
    let sorted_ranges = parse_ranges(&mut lines);
    let unique_ranges = ranges_to_non_overlapping(sorted_ranges);

    for line in &mut lines {
        let num: u64 = line.parse().expect("number");

        let found = unique_ranges
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
    let mut lines = input.lines();
    let sorted_ranges = parse_ranges(&mut lines);
    let unique_ranges = ranges_to_non_overlapping(sorted_ranges);

    let mut sol = 0;
    for range in unique_ranges {
        sol += range.end - range.start + 1;
    }

    Some(sol)
}

fn parse_ranges(lines: &mut Lines) -> Vec<Range<u64>> {
    let mut sorted_ranges = BinaryHeap::new();

    for line in lines {
        if line.is_empty() {
            break;
        }

        sorted_ranges.push(parse_range(line));
    }

    sorted_ranges.into_sorted_vec()
}

fn ranges_to_non_overlapping(sorted_ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
    let mut unique = Vec::new();
    let mut cur = sorted_ranges[0].clone();

    sorted_ranges.iter().for_each(|next| {
        if next.start <= cur.end {
            cur.end = max(next.end, cur.end);
        } else {
            unique.push(cur.clone());

            cur = next.clone();
        }
    });

    if *unique.last().unwrap() != cur {
        unique.push(cur);
    }

    unique
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
