use std::{cmp::max, ops::Sub, str::FromStr};

advent_of_code::solution!(9);

#[derive(Clone, Copy, PartialEq, Debug, Eq, PartialOrd, Ord)]
struct Point {
    j: i64,
    i: i64,
}

impl Point {
    fn area(&self, o: &Point) -> i64 {
        let rect = &self.abs_dif(o);

        (rect.j + 1) * (rect.i + 1)
    }

    fn abs_dif(&self, o: &Point) -> Point {
        Point {
            j: self.j.abs_diff(o.j) as i64,
            i: self.i.abs_diff(o.i) as i64,
        }
    }

    #[inline]
    fn _cross(&self, o: &Point) -> i64 {
        return self.j * o.i - self.i * o.j;
    }
}

fn _ccw(a: &Point, b: &Point, c: &Point) -> bool {
    (b - a)._cross(&(c - a)) > 0
}
impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            j: self.j - rhs.j,
            i: self.i - rhs.i,
        }
    }
}

#[derive(Debug)]
struct _PointParseErr {}

impl FromStr for Point {
    type Err = _PointParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair = s.split_once(',').expect("pair");

        Ok(Point {
            j: pair.0.parse().expect("x"),
            i: pair.1.parse().expect("y"),
        })
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let points = get_points(input);

    let n = points.len();
    let mut best = 0;

    for i in 0..n {
        for j in i + 1..n {
            best = max(best, points[i].area(&points[j]));
        }
    }

    Some(best)
}

pub fn part_two(_input: &str) -> Option<i64> {
    None
}

fn get_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(Point::from_str)
        .map(|r| r.expect("is point"))
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
        let _result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(24));
    }
}
