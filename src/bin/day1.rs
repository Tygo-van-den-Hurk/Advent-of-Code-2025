/// Instruction of which side to go, and how much.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instruction {
    Left(i32),
    Right(i32),
}

pub fn parse(input: &str) -> Vec<Instruction> {
    let mut result = Vec::new();
    for (index, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        let amount = chars.as_str().parse::<i32>().unwrap();

        let instruction = match direction {
            'R' | 'r' => Instruction::Right(amount),
            'L' | 'l' => Instruction::Left(amount),
            _ => panic!("Unexpected direction: '{direction}' on line {index}: '{line}'."),
        };

        println!("Parsed '{line}' into '{instruction:?}'");
        result.push(instruction);
    }

    return result;
}

pub mod part1 {
    use super::{Instruction, parse};

    pub fn compute(input: Vec<Instruction>) -> i32 {
        let mut zeros = 0;

        let mut position = 50;

        for instruction in input {

            let old = position.clone();

            match instruction {
                Instruction::Left(amount) => {
                    position = position - (amount%100);
                    if position < 0 {
                        position = 100 + position;
                    }
                },
                Instruction::Right(amount) => {
                    position = (position + amount) % 100;
                },
            }

            if position == 0 {
                zeros += 1;
            }

            println!("Using instruction '{instruction:?}' from '{old}' leads you to '{position}' with now '{zeros}' zeros.");
        }

        return zeros;
    }

    /// Returns the computation from the input
    pub fn solve(input: &str) -> i32 {
        return compute(parse(input));
    }
}

fn main() {
    const INPUT: &str = include_str!("../input/day1.txt");
    let input = parse(INPUT);

    let result1 = part1::compute(input);
    println!("The solution to part 1 is {result1}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;

    #[test]
    fn test_part1() {
        let result = part1::solve(INPUT);
        let expected = 3;
        assert_eq!(result, expected);
    }
}
