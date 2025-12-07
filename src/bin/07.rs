advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    Some(beam(input).0)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(beam(input).1)
}

/// Can make it faster if not parsing every byte.
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
fn beam(input: &str) -> (u64, u64) {
    let lines: Vec<_> = input.lines().map(str::as_bytes).collect();
    let start = lines[0].iter().position(|&b| b == b'S').expect("has start");
    let n = lines[0].len();

    let mut splits = 0;

    let mut current = vec![0; n];
    let mut next = vec![0; n];

    current[start] = 1;

    for bytes in lines {
        for (j, &count) in current.iter().enumerate() {
            if count > 0 {
                if bytes[j] == b'^' {
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
