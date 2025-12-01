advent_of_code::solution!(1);

const LEFT: char = 'L';

type Dial = i32;

#[derive(Debug)]
pub struct State {
    size: Dial,
    position: Dial,
    times_ends_at_zero: u64,
    times_crosses_zero: u64,
}

impl State {
    pub fn new(size: Dial, start: Dial) -> Self {
        Self {
            size,
            position: start,
            times_ends_at_zero: 0,
            times_crosses_zero: 0,
        }
    }

    pub fn rotate(&mut self, rotation: Dial) {
        let raw_next = self.position + rotation;

        let raw_next_as_right = if rotation >= 0 {
            raw_next
        } else {
            self.reversed_position() - rotation
        };

        let next = raw_next.rem_euclid(self.size);

        self.times_crosses_zero += (raw_next_as_right / self.size) as u64;
        self.times_ends_at_zero += u64::from(next == 0);

        self.position = next;
    }

    pub fn reversed_position(&mut self) -> Dial {
        (self.size - self.position) % self.size
    }
}

fn parse_rotations(line: &str) -> Dial {
    let mut chars = line.chars();

    let sign = match chars.next().expect("direction").eq(&LEFT) {
        true => -1,
        false => 1,
    };

    let clicks = chars.as_str().parse::<Dial>().expect("clicks");

    sign * clicks
}

pub fn solve(state: &mut State, input: &str) {
    input
        .lines()
        .map(parse_rotations)
        .for_each(|rotation| state.rotate(rotation));
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut state = State::new(100, 50);

    solve(&mut state, input);

    Some(state.times_ends_at_zero)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut state = State::new(100, 50);

    solve(&mut state, input);

    Some(state.times_crosses_zero)
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
        assert_eq!(result, Some(6));
    }
}
