use std::cmp::max;

advent_of_code::solution!(9);

type Coord = (u64, u64);

pub fn part_one(input: &str) -> Option<u64> {
    let coords = get_coords(input);

    let n = coords.len();
    let mut best = 0;

    for i in 0..n {
        for j in i + 1..n {
            let (a_x, a_y) = coords[i];
            let (b_x, b_y) = coords[j];
            let cur = (a_x.abs_diff(b_x) + 1) * (a_y.abs_diff(b_y) + 1);

            best = max(best, cur);
        }
    }

    Some(best)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn get_coords(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            let pair = line.split_once(',').expect("pair");

            (pair.0.parse().expect("x"), pair.1.parse().expect("y"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
