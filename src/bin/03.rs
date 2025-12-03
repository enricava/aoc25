use std::cmp::Ordering;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    solve_joltage(input, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve_joltage(input, 12)
}

pub fn solve_joltage(input: &str, joltage_nums: usize) -> Option<u64> {
    let joltage: u64 = input
        .lines()
        .map(|line| -> u64 {
            let nums: Vec<u8> = line.bytes().map(|b| b - b'0').collect();

            let mut sol = 0;
            let mut start = 0;

            let n = nums.len();

            // Finds the first maximum number in the vec, leaving space for the
            // remaining voltages.
            for i in (0..joltage_nums).rev() {
                let slice = &nums[0..n - i];

                // Index and maximum, where the maximum is the first maximum
                // found after `start`.
                let i_max = slice
                    .iter()
                    .enumerate()
                    .skip(start)
                    .max_by(|(a_i, a_v), (b_i, b_v)| match a_v.cmp(b_v) {
                        Ordering::Greater => Ordering::Greater,
                        Ordering::Less => Ordering::Less,
                        _ => b_i.cmp(a_i),
                    })
                    .expect("bad size");

                // Add the next maximum, start from the next maximum's index.
                start = i_max.0 + 1;
                sol = sol * 10 + (*i_max.1) as u64;
            }

            sol
        })
        .sum();

    Some(joltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
