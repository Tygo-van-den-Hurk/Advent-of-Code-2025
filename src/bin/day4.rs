#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Paper,
    Empty,
}

impl Tile {
    pub const PAPER: char = '@';
    pub const EMPTY: char = '.';
}
impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", match self {
            Tile::Empty => Tile::EMPTY,
            Tile::Paper => Tile::PAPER,
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct FloorLayout(Vec<Vec<Tile>>);

impl std::ops::Deref for FloorLayout {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for FloorLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for FloorLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.iter() {
            for tile in row {
                write!(f, "{tile}")?;
            }
            write!(f, "\n")?;
        }

        return Ok(());
    }
}

impl AsRef<Vec<Vec<Tile>>> for FloorLayout {
    fn as_ref(&self) -> &Vec<Vec<Tile>> {
        &self.0
    }
}

impl AsMut<Vec<Vec<Tile>>> for FloorLayout {
    fn as_mut(&mut self) -> &mut Vec<Vec<Tile>> {
        &mut self.0
    }
}

impl Into<Vec<Vec<Tile>>> for FloorLayout {
    fn into(self) -> Vec<Vec<Tile>> {
        self.0
    }
}

impl<'a> IntoIterator for &'a FloorLayout {
    type Item = &'a Vec<Tile>;
    type IntoIter = std::slice::Iter<'a, Vec<Tile>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut FloorLayout {
    type Item = &'a mut Vec<Tile>;
    type IntoIter = std::slice::IterMut<'a, Vec<Tile>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl IntoIterator for FloorLayout {
    type Item = Vec<Tile>;
    type IntoIter = std::vec::IntoIter<Vec<Tile>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FloorLayout {
    pub fn new() -> FloorLayout {
        FloorLayout(Vec::new())
    }
}

pub fn parse(input: &str) -> FloorLayout {
    let mut result = FloorLayout::new();
    for (line_index, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let mut row = Vec::new();
        for (column_index, char) in line.chars().enumerate() {
            row.push(match char {
                Tile::EMPTY => Tile::Empty,
                Tile::PAPER => Tile::Paper,
                _ => panic!(
                    "Expected either '{}', or '{}' but found '{char}' on line {line_index} column{column_index}.", Tile::EMPTY, Tile::PAPER
                )
            });
        }

        result.push(row);
    }

    println!("Parsed input string:{input}\nas the following scheme:\n{result}\n");
    return result;
}

fn amount_of_neighbors(neighborhood: &FloorLayout, (y, x): (usize, usize)) -> usize {
    let mut total = 0;

    let y = i32::try_from(y).unwrap();
    let x = i32::try_from(x).unwrap();

    for row_mod in -1..=1 {
        for col_mod in -1..=1 {
            let y = y + row_mod;
            let x = x + col_mod;

            println!("\t(y, x) == ({y}, {x})");

            if row_mod == 0 && col_mod == 0 {
                println!("is current tile");
                continue;
            }

            if y < 0 {
                continue;
            }

            if y >= neighborhood.len().try_into().unwrap() {
                continue;
            }

            if x < 0 {
                continue;
            }

            if x >= neighborhood[y as usize].len().try_into().unwrap() {
                continue;
            }

            print!("\tneighborhood[y][x] == {:?}", neighborhood[y as usize][x as usize]);

            if let Tile::Paper = neighborhood[y as usize][x as usize] {
                total += 1;
                println!("\t(total = {total})");
            } else {
                println!("");
            }
        }
    }

    return total;
}

pub mod part1 {
    use super::{Tile, parse, FloorLayout, amount_of_neighbors};

    pub fn compute(input: &FloorLayout) -> u64 {
        let mut total = 0;

        println!("neighborhood = ({}x{})", input.len(), input[0].len());
        for (y, row) in input.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                println!("{tile} at (y={y}, x={x}) is {tile:?}");
                if let Tile::Paper = *tile && amount_of_neighbors(input, (y, x)) < 4 {
                    total += 1;
                }
            }
        }

        return total;
    }

    /// Returns the computation from the input
    pub fn solve(input: &str) -> u64 {
        return compute(&parse(input));
    }
}


fn main() {
    const INPUT: &str = include_str!("../input/day4.txt");
    let input = parse(INPUT);

    let result1 = part1::compute(&input);
    println!("The solution to part 1 is {result1}");
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

    #[test]
    fn test_part1() {
        let result = part1::solve(INPUT);
        let expected = 13;
        assert_eq!(result, expected);
    }
}
