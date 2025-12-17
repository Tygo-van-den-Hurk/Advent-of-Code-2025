/// Instruction of which side to go, and how much.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Range {
    start: u64,
    stop: u64,
}

impl Range {
    pub fn new(start: u64, stop: u64) -> Self {
        return Range { start, stop };
    }

    pub fn iter(&self) -> impl Iterator<Item = u64> {
        return self.start..self.stop+1;
    }
}

pub fn parse(input: &str) -> Vec<Range> {
    let mut result = Vec::new();
    for entry in input.split(',').map(|str| str.trim()) {
        if entry.trim().is_empty() {
            continue;
        }

        let split = entry.split_once('-');
        if split.is_none() {
            panic!("entry '{entry}' does not contain a '-' char.")
        }

        let (left, right) = split.unwrap();

        let left_parsed = left.trim().parse();
        if left_parsed.is_err() {
            panic!("Could not convert '{left}' into a i32");
        }

        let right_parsed = right.trim().parse();
        if right_parsed.is_err() {
            panic!("Could not convert '{right}' into a i32");
        }

        let range = Range::new(left_parsed.unwrap(), right_parsed.unwrap());
        println!("Parsed '{entry}' into '{range:?}'.");
        result.push(range);
    }

    return result;
}

pub mod part1 {
    use super::{Range, parse};

    /// Checks if an ID is valid.
    pub fn is_invalid_id(id: u64) -> bool {
        let str = id.to_string();
        let len = str.len();
        let (left, right) = str.split_at(len / 2);
        return left == right;
    }

    pub fn compute(input: Vec<Range>) -> u64 {
        let mut count = 0;

        for range in input {
            println!("Looking through {range:?} for invalid IDs:");
            for number in range.iter() {
                if is_invalid_id(number) {
                    println!("\tFound {number} in {range:?} total: {count}");
                    count += number;
                }
            }
        }

        return count;
    }

    /// Returns the computation from the input
    pub fn solve(input: &str) -> u64 {
        return compute(parse(input));
    }
}

pub mod part2 {
    use super::{Range, parse};

    /// Checks if an ID is valid.
    pub fn is_invalid_id(id: u64) -> bool {
        let str = id.to_string();
        let possible_substring_lengths = 1..(str.len()/2)+1;
        let numbers: Vec<u64> = str.chars().map(|c| c.to_string().parse().unwrap()).collect();

        println!("\tpossible_substring_lengths: {possible_substring_lengths:?} in {str}");
        for length in possible_substring_lengths {
            if str.len() % length != 0 { continue; }

            let mut found = true;
            let substring1 = numbers.get(0..length).unwrap();
            println!("\t\tlength: {length}, substring1: {substring1:?}");
            for index in 1..(numbers.len()/length) {
                let next_slice_indexes = index*length..(1+index)*length;
                println!("\t\t\tLooking for numbers in range: {next_slice_indexes:?}");
                let substring2 = numbers.get(next_slice_indexes).unwrap();
                println!("\t\t\tcomparing '{substring1:?}' with '{substring2:?}' in {str}");
                if substring2 != substring1 {
                    found = false;
                    println!("\t\t\t\tThey are not the same");
                    break;
                }
            }

            if found {
                println!("\tfound! in {str} with length {length}");
                return true;
            }
        }

        println!("\tnot found!");
        return false;
    }

    pub fn compute(input: Vec<Range>) -> u64 {
        let mut count = 0;

        for range in input {
            println!("Looking through {range:?} for invalid IDs:");
            for number in range.iter() {
                if is_invalid_id(number) {
                    println!("\tFound {number} in {range:?} total: {count}");
                    count += number;
                }
            }
        }

        return count;
    }

    /// Returns the computation from the input
    pub fn solve(input: &str) -> u64 {
        return compute(parse(input));
    }
}

fn main() {
    const INPUT: &str = include_str!("../input/day2.txt");
    let input = parse(INPUT);

    let result1 = part1::compute(input.clone());
    println!("The solution to part 1 is {result1}");

    let result2 = part2::compute(input);
    println!("The solution to part 2 is {result2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn test_part1() {
        let result = part1::solve(INPUT);
        let expected = 1227775554;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let result = part2::solve(INPUT);
        let expected = 4174379265;
        assert_eq!(result, expected);
    }
}
