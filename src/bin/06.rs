use std::{ops::ControlFlow, str::FromStr};

advent_of_code::solution!(6);

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug)]
struct OpErr;

impl FromStr for Op {
    type Err = OpErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('+') => Ok(Op::Add),
            Some('*') => Ok(Op::Mul),
            _ => Err(self::OpErr),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut mat = Vec::new();
    let mut ops: Vec<Op> = Vec::new();

    input.lines().for_each(|line| {
        let mut row: Vec<u64> = Vec::new();

        let res = line.split_whitespace().try_for_each(|num| {
            if let Ok(n) = num.parse() {
                row.push(n);
                return ControlFlow::Continue(());
            }

            ControlFlow::Break(())
        });

        match res {
            ControlFlow::Continue(_) => mat.push(row),
            ControlFlow::Break(_) => {
                line.split_whitespace()
                    .for_each(|op| ops.push(op.parse().expect("should be op")));
            }
        }
    });

    let m = mat.len();
    let n = mat[0].len();
    let mut sol = 0;

    for j in 0..n {
        let op = &ops[j];

        let res = match op {
            Op::Add => {
                let mut acc = 0;
                for i in 0..m {
                    acc += mat[i][j]
                }
                acc
            }
            Op::Mul => {
                let mut acc = 1;
                for i in 0..m {
                    acc *= mat[i][j]
                }
                acc
            }
        };

        sol += res;
    }

    Some(sol)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut mat = Vec::new();
    let mut ops: Vec<Op> = Vec::new();

    input.lines().for_each(|line| {
        let mut row: Vec<Option<u8>> = Vec::new();

        for b in line.bytes() {
            if b.is_ascii_digit() {
                row.push(Some(b - b'0'));
            } else if b.is_ascii_whitespace() {
                row.push(None);
            } else {
                break;
            }
        }

        if !row.is_empty() {
            // Add the row
            mat.push(row);
        } else {
            // last line, add the ops
            line.split_whitespace()
                .for_each(|op| ops.push(op.parse().expect("should be op")));
        }
    });

    let batches = to_batches(mat);
    assert_eq!(batches.len(), ops.len());

    let sol = batches.iter().zip(ops).fold(0, |acc, (batch, op)| {
        acc + match op {
            Op::Add => batch.iter().fold(0, |a, n| a + n),
            Op::Mul => batch.iter().fold(1, |a, n| a * n),
        }
    });

    Some(sol)
}

fn to_batches(mat: Vec<Vec<Option<u8>>>) -> Vec<Vec<u64>> {
    let m = mat.len();
    let n = mat[0].len();

    let mut batches = Vec::new();
    let mut batch = Vec::new();

    for j in 0..n {
        let mut num: u64 = 0;
        let mut is_empty = true;

        for i in 0..m {
            if let Some(d) = mat[i][j] {
                num = num * 10 + d as u64;
                is_empty = false;
            }
        }

        if !is_empty {
            batch.push(num);
        } else {
            batches.push(batch);
            batch = Vec::new();
        }
    }
    batches.push(batch);

    batches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
