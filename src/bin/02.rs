use std::collections::HashSet;

advent_of_code::solution!(2);

/// Inclusive [Range] over [T].
struct Range<T>
where
    T: PartialOrd,
{
    start: T,
    end: T,
}

impl<T: PartialOrd> Range<T> {
    fn has(&self, item: T) -> bool {
        return self.start <= item && item <= self.end;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    const PARTS: u8 = 2;

    let sum = input
        .lines()
        .next()
        .expect("at least one line")
        .split(',')
        .map(parse_range)
        .fold(0, |acc, next_range| {
            acc + sum_invalids(&next_range, PARTS, &mut None)
        });

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    let mut set = Some(&mut HashSet::new());

    let ranges: Vec<Range<u64>> = input
        .lines()
        .next()
        .expect("at least one line")
        .split(',')
        .map(parse_range)
        .collect();

    for parts in 2..8 {
        sum += ranges.iter().fold(0, |acc, next_range| {
            acc + sum_invalids(next_range, parts, &mut set)
        });
    }

    Some(sum)
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

fn sum_invalids(range: &Range<u64>, num_parts: u8, set: &mut Option<&mut HashSet<u64>>) -> u64 {
    let mut sum = 0;

    let start_digits = digits(range.start);
    let end_digits = digits(range.end);

    for digits in start_digits..=end_digits {
        if digits % num_parts != 0 {
            continue;
        }

        let part_size = digits / num_parts;

        let mut first_part = prefix(range.start, part_size);
        let mut last_part = prefix(range.end, part_size);

        if last_part < first_part {
            first_part = u64::pow(10, part_size as u32 - 1);
            last_part = u64::pow(10, part_size as u32) - 1;
        }

        for part in first_part..=last_part {
            let invalid = compose(part, part_size, num_parts);

            if range.has(invalid) {
                match set {
                    Some(s) => {
                        if !s.contains(&invalid) {
                            s.insert(invalid);
                            sum += invalid;
                        }
                    }
                    None => {
                        sum += invalid;
                    }
                }
            }
        }
    }

    sum
}

fn digits(num: u64) -> u8 {
    let mut digits = 0;
    let mut n = num;

    while n > 0 {
        n = n / 10;
        digits += 1;
    }

    digits
}

fn prefix(num: u64, prefix_digits: u8) -> u64 {
    if num == 0 {
        return 0;
    }

    num / u64::pow(10, (digits(num) - prefix_digits) as u32)
}

fn compose(part: u64, part_size: u8, num_parts: u8) -> u64 {
    debug_assert_eq!(digits(part), part_size);

    let mut invalid = part;
    let mut n = num_parts;

    while n > 1 {
        invalid *= u64::pow(10, part_size.into());
        invalid += part;
        n -= 1;
    }

    invalid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        assert_eq!(digits(0), 0);
        assert_eq!(digits(1), 1);
        assert_eq!(digits(1234), 4);
    }

    #[test]
    fn test_prefix() {
        assert_eq!(prefix(0, 1), 0);
        assert_eq!(prefix(1, 1), 1);
        assert_eq!(prefix(1234, 1), 1);
        assert_eq!(prefix(1234, 2), 12);
        assert_eq!(prefix(1234, 3), 123);
        assert_eq!(prefix(1234, 4), 1234);
    }

    #[test]
    fn test_compose() {
        assert_eq!(compose(1, 1, 0), 1);
        assert_eq!(compose(1, 1, 1), 1);
        assert_eq!(compose(1, 1, 2), 11);
        assert_eq!(compose(123, 3, 1), 123);
        assert_eq!(compose(123, 3, 2), 123123);
        assert_eq!(compose(123, 3, 3), 123123123);
        assert_eq!(compose(123, 3, 4), 123123123123);
    }

    #[test]
    fn test_edge_case() {
        let r = Range {
            start: 200,
            end: 1100,
        };

        assert_eq!(sum_invalids(&r, 2, &mut None), 1010);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
