# Advent of Code 2025

Practising my acquired knowledge of `rust`.

*⚠️ NOTE ⚠️* Most of these solutions are uploaded without any sort of cleanup, so
lower expectations on code readibility and abstraction.

Using the [advent-of-code-rust](https://github.com/fspoettel/advent-of-code-rust)
template.

## Running

```fish
# Simplest way to scaffold a day.
cargo today

# Scaffold a day and download its input.
cargo scaffold <day> --download

# Solve / submit.
cargo solve <day> [--release] [--submit]

# Time.
cargo time <day> [--all] [--store]
```

## Testing

```fish
cargo test
```

## When will I get bored?

- [x] Day1
  - [x] Part 1: 25.6µs @ 10000 samples
  - [x] Part 2: 23.1µs @ 10000 samples
- [x] Day2
  - [x] Part 1: 4.9µs @ 10000 samples
  - [x] Part 2: 17.1µs @ 10000 samples
- [x] Day3
  - [x] Part 1: 79.5µs @ 5043 samples
  - [x] Part 2: 129.5µs @ 5177 samples
- [x] Day4
  - [x] Part 1: 572.1µs @ 879 samples
  - [x] Part 2: 1.4ms @ 669 samples
- [x] Day5
  - [x] Part 1: 44.4µs @ 5347 samples
  - [x] Part 2: 10.4µs @ 10000 samples
- [x] Day6
  - [x] Part 1: 32.3µs @ 7305 samples
  - [x] Part 2: 43.9µs @ 9732 samples
- [x] Day7
  - [x] Part 1: 14.3µs @ 10000 samples
  - [x] Part 2: 11.3µs @ 10000 samples
  - Removing the parsing step made it 4x faster, huh.
- [x] Day8
  - [x] Part 1: 619.1µs @ 519 samples
  - [x] Part 2: 38.8ms @ 24 samples
  - Union-find & Kruskal ftw! Needs cleanup.
- [ ] Day9
  - [x] Part 1: 77.5µs @ 10000 samples
  - Huuh, tried some convex hull stuff, but that does not make sense.
  - What this needs is the contour, I think, so Moore Neighbor tracing and check
  if every corner is inside?
- [ ] Day10
  - [x] Part 1: 9.3ms @ 47 samples
  - Did not read properly.
  - Replaced my vecs with bitsets and now I need vecs for part 2...
- [ ] Day11
  - [x] Part 1: 120.9µs @ 3290 samples
- [ ] Day12
