/// The puzzle input created from text.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct PuzzleInput {
    ranges: Vec<std::ops::Range<usize>>,
    ids: Vec<usize>,
}

impl PuzzleInput {
    pub fn new() -> PuzzleInput {
        PuzzleInput::default()
    }
}

impl std::str::FromStr for PuzzleInput {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iter = input.trim().lines();

        let mut result = Self::default();

        // Build ranges
        for line in &mut iter {
            if line.trim().is_empty() {
                break;
            }

            let (left, right) = line.trim().split_once('-').expect(&format!("{line} does not contain a '-' char."));


            let start = left.parse::<usize>().unwrap();
            let stop = 1 + right.parse::<usize>().unwrap();
            result.ranges.push(start..stop);
        }

        for line in &mut iter {
            if line.trim().is_empty() {
                break;
            }

            result.ids.push(line.trim().parse().unwrap());
        }

        return Result::Ok(result);
    }
}

pub mod part1 {
    use super::PuzzleInput;

    pub fn compute(input: PuzzleInput) -> i32 {
        let mut fresh = 0;

        for id in & input.ids {
            for range in & input.ranges {
                if range.contains(&id) {
                    println!("id {id} is in range of {range:?}");
                    fresh += 1;
                    break;
                }
            }
        }

        return fresh;
    }

    /// Returns the computation from the input
    pub fn solve(input: &str) -> i32 {
        return compute(input.parse().unwrap());
    }
}

pub mod part2 {
    use super::PuzzleInput;

    pub fn compute(mut input: PuzzleInput) -> usize {
        input.ranges.sort_by_key(|range| range.start);
        let mut merged: Vec<std::ops::Range<usize>> = Vec::new();
        for r in input.ranges {
            if let Some(last) = merged.last_mut() {
                if r.start <= last.end {
                    last.end = last.end.max(r.end);
                } else {
                    merged.push(r);
                }
            } else {
                merged.push(r);
            }
        }

        merged.iter().map(|r| r.end - r.start).sum()
    }

    /// Returns the computation from the input
    pub fn solve(input: &str) -> usize {
        return compute(input.parse().unwrap());
    }
}

fn main() {
    const INPUT: &str = include_str!("../input/day5.txt");
    let input: PuzzleInput = INPUT.parse().unwrap();

    let result1 = part1::compute(input.clone());
    println!("The solution to part 1 is {result1}");

    let result1 = part2::compute(input);
    println!("The solution to part 2 is {result1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

    #[test]
    fn test_part1() {
        let result = part1::solve(INPUT);
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let result = part2::solve(INPUT);
        let expected = 14;
        assert_eq!(result, expected);
    }
}
