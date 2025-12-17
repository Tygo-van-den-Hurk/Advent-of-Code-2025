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
                panic!(
                    "Could not convert char {char} on line {line_index} column {column_index} to a u8."
                )
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

    impl From<BatteryPack> for u64 {
        fn from(value: BatteryPack) -> Self {
            10 * (value.0 as u64) + value.1 as u64
        }
    }

    pub fn compute(input: &Vec<Vec<Battery>>) -> u64 {
        let mut total = 0;
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
            total += u64::from(combination);
        }

        return total;
    }

    /// Returns the computation from the input
    pub fn solve(input: &str) -> u64 {
        return compute(&parse(input));
    }
}

pub mod part2 {

    use super::{Battery, parse};

    /// A collection of `Battery`s to maximize the combined power output of.
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
    pub struct BatteryPack([Battery; BatteryPack::SIZE]);

    impl std::ops::Deref for BatteryPack {
        type Target = [Battery; BatteryPack::SIZE];

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for BatteryPack {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl BatteryPack {
        /// The size of the `BatteryPack`.
        pub const SIZE: usize = 12;

        pub fn new(value: [Battery; BatteryPack::SIZE]) -> BatteryPack {
            return BatteryPack(value);
        }
    }

    impl From<[Battery; BatteryPack::SIZE]> for BatteryPack {
        fn from(value: [Battery; BatteryPack::SIZE]) -> Self {
            return BatteryPack(value);
        }
    }

    impl From<&BatteryPack> for u128 {
        fn from(value: &BatteryPack) -> Self {
            let mut total = 0;
            for (index, element) in value.iter().rev().enumerate() {
                let pow = 10_u128.pow(index as u32);
                total += pow * (*element as u128)
            }

            return total;
        }
    }

    pub fn compute(input: &Vec<Vec<Battery>>) -> u128 {
        let mut total = 0;

        for battery_bank in input {
            let n = battery_bank.len();
            let k = BatteryPack::SIZE;
            let mut stack: Vec<Battery> = Vec::with_capacity(k);

            for (i, &b) in battery_bank.iter().enumerate() {
                while !stack.is_empty() && *stack.last().unwrap() < b && stack.len() + (n - i) > k {
                    stack.pop();
                }

                if stack.len() < k {
                    stack.push(b);
                }
            }

            let mut array = [0; BatteryPack::SIZE];
            array.copy_from_slice(&stack);
            let pack = BatteryPack::new(array);

            total += u128::from(&pack);
        }

        return total;
    }

    /// Returns the computation from the input
    pub fn solve(input: &str) -> u128 {
        return compute(&parse(input));
    }
}

fn main() {
    const INPUT: &str = include_str!("../input/day3.txt");
    let input = parse(INPUT);

    let result1 = part1::compute(&input);
    println!("The solution to part 1 is {result1}");

    let result2 = part2::compute(&input);
    println!("The solution to part 2 is {result2}");
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

    #[test]
    fn test_part2_battery_pack_sum() {
        let pack = part2::BatteryPack::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
        let expected = 123456789012;
        assert_eq!(u128::from(&pack), expected);
    }

    // #[test]
    // fn test_part2() {
    //     let result = part2::solve(INPUT);
    //     let expected = 3121910778619;
    //     assert_eq!(result, expected);
    // }

    #[test]
    fn test_part2_single_row1() {
        let result = part2::solve("987654321111111");
        let expected = 987654321111;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2_single_row2() {
        let result = part2::solve("811111111111119");
        let expected = 811111111119;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2_single_row3() {
        let result = part2::solve("234234234234278");
        let expected = 434234234278;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2_single_row4() {
        let result = part2::solve("818181911112111");
        let expected = 888911112111;
        assert_eq!(result, expected);
    }
}
