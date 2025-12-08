use std::{cmp::Reverse, collections::BinaryHeap, str::FromStr};

advent_of_code::solution!(8);

#[derive(Debug)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct CoordParseErr {}

impl FromStr for Coord {
    type Err = CoordParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(3, ',').map(|part| part.parse().expect("commas"));

        Ok(Coord {
            x: parts.next().expect("x"),
            y: parts.next().expect("y"),
            z: parts.next().expect("z"),
        })
    }
}

impl Coord {
    fn dist(&self, o: &Self) -> i64 {
        return (self.x - o.x).pow(2) + (self.y - o.y).pow(2) + (self.z - o.z).pow(2);
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let coords: Vec<_> = input
        .lines()
        .map(|line| Coord::from_str(line).expect("coord"))
        .collect();

    Some(best_connections(coords, 3, 1000))
}

pub fn part_two(input: &str) -> Option<u64> {
    let coords: Vec<_> = input
        .lines()
        .map(|line| Coord::from_str(line).expect("coord"))
        .collect();

    Some(best_connections_2(coords))
}

fn find(parent: &mut Vec<usize>, x: usize) -> usize {
    if parent[x] != x {
        parent[x] = find(parent, parent[x]);
    }
    parent[x]
}

fn union(parent: &mut Vec<usize>, size: &mut Vec<usize>, x: usize, y: usize) {
    let mut x = find(parent, x);
    let mut y = find(parent, y);

    if x != y {
        if size[x] < size[y] {
            (x, y) = (y, x);
        }

        parent[y] = x;
        size[x] += size[y];
        size[y] = 0;
    }
}

fn best_connections(coords: Vec<Coord>, num_multiply_sets: usize, num_connections: usize) -> u64 {
    let n = coords.len();
    let mut min_distances = BinaryHeap::with_capacity(num_connections);

    for i in 0..n {
        let c = &coords[i];

        for j in i + 1..n {
            let o = &coords[j];
            let dist = c.dist(o);
            let item = (dist, i, j);

            if min_distances.len() < num_connections {
                min_distances.push(item);
            } else if let Some(max_distance) = min_distances.peek()
                && max_distance.0 > dist
            {
                min_distances.pop();
                min_distances.push(item);
            }
        }
    }

    let mut parent: Vec<usize> = (0..n).collect();
    let mut size: Vec<usize> = vec![1; n];

    let min_dists = min_distances.into_sorted_vec();

    for (_, i, j) in min_dists {
        union(&mut parent, &mut size, i, j);
    }

    let mut max_sizes = BinaryHeap::new();

    for i in 0..parent.len() {
        let root = find(&mut parent, i);
        let set_size = Reverse(size[root]);

        if max_sizes.len() < num_multiply_sets {
            max_sizes.push(set_size);
        } else if let Some(min_size) = max_sizes.peek()
            && min_size > &set_size
        {
            max_sizes.pop();
            max_sizes.push(set_size);
        }

        // Already seen
        size[root] = 0;
    }

    max_sizes.iter().fold(1, |a, &s| a * s.0 as u64)
}

fn best_connections_2(coords: Vec<Coord>) -> u64 {
    let n = coords.len();
    let mut distances = BinaryHeap::new();

    for i in 0..n {
        let c = &coords[i];

        for j in i + 1..n {
            let o = &coords[j];
            let dist = c.dist(o);
            let item = (dist, i, j);

            distances.push(item);
        }
    }

    let mut parent: Vec<usize> = (0..n).collect();
    let mut size: Vec<usize> = vec![1; n];
    let min_dists = distances.into_sorted_vec();

    // kruskal
    let mut last_union = 0;

    for (_, i, j) in min_dists {
        let parent_i = find(&mut parent, i);
        let parent_j = find(&mut parent, j);

        if parent_i != parent_j {
            last_union = coords[i].x * coords[j].x;
            union(&mut parent, &mut size, parent_i, parent_j);
        }
    }

    last_union as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let coords = advent_of_code::template::read_file("examples", DAY)
            .lines()
            .map(|line| Coord::from_str(line).expect("coord"))
            .collect();

        let result = best_connections(coords, 3, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
