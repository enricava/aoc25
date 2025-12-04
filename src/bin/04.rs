advent_of_code::solution!(4);

#[derive(Debug, PartialEq, Eq)]
enum CellType {
    Empty,
    Paper,
}

impl From<u8> for CellType {
    fn from(value: u8) -> Self {
        match value {
            b'.' => CellType::Empty,
            b'@' => CellType::Paper,
            _ => unreachable!(),
        }
    }
}

pub struct Cell {
    kind: CellType,
    adj: u8,
}

impl Cell {
    fn new(t: CellType) -> Self {
        Self { kind: t, adj: 0 }
    }
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        Cell::new(CellType::from(value))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut matrix = read_matrix(input);

    let m = matrix.len();
    let n = matrix[0].len();

    for i in 0..m {
        for j in 0..n {
            let cell = &mut matrix[i][j];

            if cell.kind == CellType::Paper {
                for (ki, kj) in adjs(i, j, m, n) {
                    matrix[ki][kj].adj += 1;
                }
            }
        }
    }

    let mut count = 0;

    for i in 0..m {
        for j in 0..n {
            let cell = &mut matrix[i][j];

            if cell.kind == CellType::Paper {
                if cell.adj < 4 {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut matrix = read_matrix(input);

    let m = matrix.len();
    let n = matrix[0].len();

    for i in 0..m {
        for j in 0..n {
            let cell = &mut matrix[i][j];

            if cell.kind == CellType::Paper {
                for (ki, kj) in adjs(i, j, m, n) {
                    matrix[ki][kj].adj += 1;
                }
            }
        }
    }

    let mut count_and_remove = || {
        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                let cell = &mut matrix[i][j];

                if cell.kind == CellType::Paper {
                    if cell.adj < 4 {
                        count += 1;

                        cell.kind = CellType::Empty;

                        for (ki, kj) in adjs(i, j, m, n) {
                            matrix[ki][kj].adj -= 1;
                        }
                    }
                }
            }
        }

        count
    };

    let mut count = 0;

    loop {
        let next = count_and_remove();

        count += next;

        if next == 0 {
            break;
        }
    }

    Some(count)
}

fn adjs(i: usize, j: usize, m: usize, n: usize) -> Vec<(usize, usize)> {
    const DIRECTIONS: [(i8, i8); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ];

    let mut adjs = Vec::new();

    for (x, y) in DIRECTIONS {
        let (ki, kj) = (i as i32 + x as i32, j as i32 + y as i32);

        if inside(ki, kj, m, n) {
            adjs.push((ki as usize, kj as usize));
        }
    }

    adjs
}

fn inside(ki: i32, kj: i32, m: usize, n: usize) -> bool {
    0 <= ki && ki < m as i32 && 0 <= kj && kj < n as i32
}

pub fn read_matrix(input: &str) -> Vec<Vec<Cell>> {
    let mut matrix = Vec::new();

    input.lines().for_each(|line| {
        let mut row = Vec::new();

        line.bytes().for_each(|c| row.push(Cell::from(c)));

        matrix.push(row);
    });

    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
