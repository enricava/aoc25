advent_of_code::solution!(7);

#[derive(Debug, Clone)]
struct Coord {
    i: usize,
    j: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellType {
    Empty,
    Source,
    Splitter,
}

impl CellType {
    fn from_byte(b: u8) -> Self {
        match b {
            b'.' => CellType::Empty,
            b'S' => CellType::Source,
            b'^' => CellType::Splitter,
            _ => unreachable!(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut mat, start) = read_manifold(input);

    Some(beam(&mut mat, &start).0)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut mat, start) = read_manifold(input);

    Some(beam(&mut mat, &start).1)
}

///
/// p(i, j) = {
///   i >= n || j >= m? 0,
///   s(i - 1, j) ? p(i-1, j-1) + p(i-1, j+1) : p(i-1, j)
/// }
///
/// therefore
///
/// p(i+1, j) = {
///   s(i, j) ? p(i, j-1) + p(i, j+1) : p(i, j)
/// }
///
/// only need one additional row.
fn beam(mat: &mut Vec<Vec<CellType>>, start: &Coord) -> (u64, u64) {
    let m = mat.len();
    let n = mat[0].len();

    let mut splits = 0;

    let mut current = vec![0; n];
    let mut next = vec![0; n];

    current[start.j] = 1;

    for i in 0..m {
        let row = &mat[i];

        for (j, &count) in current.iter().enumerate() {
            if count > 0 {
                if row[j] == CellType::Splitter {
                    splits += 1;

                    if j - 1 < n {
                        next[j - 1] += count;
                    }

                    if j + 1 < n {
                        next[j + 1] += count;
                    }
                } else {
                    next[j] += count;
                }
            }
        }

        (current, next) = (next, current);
        next.fill(0);
    }

    (splits, current.iter().sum())
}

fn read_manifold(input: &str) -> (Vec<Vec<CellType>>, Coord) {
    let mut mat: Vec<Vec<CellType>> = Vec::new();

    let mut cur = Coord { i: 0, j: 0 };
    let mut start = Coord { i: 0, j: 0 };

    for line in input.lines() {
        let mut row = Vec::new();

        for b in line.bytes() {
            let t = CellType::from_byte(b);

            if t == CellType::Source {
                start = cur.clone();
            }

            row.push(t);

            cur.j += 1;
        }

        mat.push(row);
        cur.i += 1;
    }

    (mat, start.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
