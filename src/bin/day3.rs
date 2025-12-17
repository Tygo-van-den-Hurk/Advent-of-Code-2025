type Battery = u8;

pub fn parse(input: &str) -> Vec<Vec<Battery>> {
    let mut result = Vec::new();
    for (line_index, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let mut row = Vec::new();
        for (column_index, char) in line.chars().enumerate() {
            if let Result::Ok(battery) = char.to_string().parse() {
                row.push(battery);
            } else {
                panic!("Could not convert char {char} on line {line_index} column {column_index} to a u8.")
            }
        }

        result.push(row);
    }

    return result;
}

pub mod part1 {
    use super::{Battery, parse};


    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct BatteryPack(Battery, Battery);

    impl From<(u8, u8)> for BatteryPack {
        fn from(value: (u8, u8)) -> Self {
            return BatteryPack(value.0, value.1);
        }
    }

    impl From<BatteryPack> for u32 {
        fn from(value: BatteryPack) -> Self {
            10*(value.0 as u32) + value.1 as u32
        }
    }

    pub fn compute(input: &Vec<Vec<Battery>>) -> u32 {

        let mut total: u32 = 0;
        for battery_bank in input {

            let mut combination = BatteryPack(0, 0);
            for battery in battery_bank.iter().rev() {
                if combination.1 == 0 {
                    combination.1 = *battery;
                } else if *battery >= combination.0 {
                    let old = combination.0;
                    combination.0 = *battery;
                    if old > combination.1 {
                        combination.1 = old;
                    }
                }
            }

            println!("The highest number out of {battery_bank:?} we could make is {combination:?}");
            total += u32::from(combination);
        }

        return total;
    }

    /// Returns the computation from the input
    pub fn solve(input: &str) -> u32 {
        return compute(&parse(input));
    }
}

fn main() {
    const INPUT: &str = include_str!("../input/day3.txt");
    let input = parse(INPUT);

    let result1 = part1::compute(&input);
    println!("The solution to part 1 is {result1}")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
987654321111111
811111111111119
234234234234278
818181911112111
"#;

    #[test]
    fn test_part1() {
        let result = part1::solve(INPUT);
        let expected = 357;
        assert_eq!(result, expected);
    }
}
