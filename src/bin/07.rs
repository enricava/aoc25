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

#[derive(Debug, Clone)]
struct Cell {
    is_splitter: bool,
    visited: bool,
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

    let m = mat.len();
    let n = mat[0].len();

    Some(count_splits(&mut mat, &start, m, n))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut mat, start) = read_manifold(input);

    let m = mat.len();
    let n = mat[0].len();

    Some(count_paths_naive(&mut mat, &start, m, n))
}

fn count_splits(mat: &mut Vec<Vec<Cell>>, cur: &Coord, m: usize, n: usize) -> u64 {
    let mut splits = 0;

    let cell = &mut mat[cur.i][cur.j];
    cell.visited = true;

    let adjs = if cell.is_splitter {
        splits += 1;

        vec![nextl(cur), nextr(cur)]
    } else {
        vec![next(cur)]
    };

    for adj in adjs {
        if inside(&adj, m, n) {
            let next = &mut mat[adj.i][adj.j];

            if !next.visited {
                splits += count_splits(mat, &adj, m, n);
            }
        }
    }
    splits
}

fn count_paths_naive(mat: &mut Vec<Vec<Cell>>, cur: &Coord, m: usize, n: usize) -> u64 {
    let mut paths = u64::from(cur.i == m - 1);

    let cell = &mut mat[cur.i][cur.j];
    cell.visited = true;

    let adjs = if cell.is_splitter {
        vec![nextl(cur), nextr(cur)]
    } else {
        vec![next(cur)]
    };

    for adj in adjs {
        if inside(&adj, m, n) {
            let next = &mut mat[adj.i][adj.j];

            if !next.visited {
                paths += count_paths_naive(mat, &adj, m, n);
            }
        }
    }

    mat[cur.i][cur.j].visited = false;

    paths
}

fn next(c: &Coord) -> Coord {
    Coord { i: c.i + 1, j: c.j }
}

fn nextl(c: &Coord) -> Coord {
    Coord {
        i: c.i + 1,
        j: c.j.overflowing_sub(1).0,
    }
}

fn nextr(c: &Coord) -> Coord {
    Coord {
        i: c.i + 1,
        j: c.j + 1,
    }
}

fn inside(c: &Coord, m: usize, n: usize) -> bool {
    return c.i < m && c.j < n;
}

fn read_manifold(input: &str) -> (Vec<Vec<Cell>>, Coord) {
    let mut mat: Vec<Vec<Cell>> = Vec::new();

    let mut cur = Coord { i: 0, j: 0 };
    let mut start = Coord { i: 0, j: 0 };

    for line in input.lines() {
        let mut row = Vec::new();

        for b in line.bytes() {
            let t = CellType::from_byte(b);

            row.push(Cell {
                is_splitter: t == CellType::Splitter,
                visited: false,
            });

            if t == CellType::Source {
                start = cur.clone();
            }

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
