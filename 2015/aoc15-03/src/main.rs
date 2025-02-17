use anyhow::*;
use aoc_common::*;
use std::collections::HashSet;
use std::str::FromStr;

fn main() -> Result<()> {
    run_vec(parse_chars, part1, part2)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum CardinalDirection {
    North,
    South,
    East,
    West,
}

impl FromStr for CardinalDirection {
    type Err = Error;

    fn from_str(direction: &str) -> Result<Self, Self::Err> {
        match direction {
            "^" => Ok(CardinalDirection::North),
            ">" => Ok(CardinalDirection::East),
            "v" => Ok(CardinalDirection::South),
            "<" => Ok(CardinalDirection::West),
            unknown => bail!("unknown direction '{}'", unknown),
        }
    }
}

fn move_santa(location: IPoint2D, direction: CardinalDirection) -> IPoint2D {
    match direction {
        CardinalDirection::North => location.up(),
        CardinalDirection::East => location.right(),
        CardinalDirection::South => location.down(),
        CardinalDirection::West => location.left(),
    }
}

fn part1(directions: &[CardinalDirection]) -> Result<usize> {
    let mut visited = HashSet::new();
    let mut location = IPoint2D::ORIGIN;
    visited.insert(location);
    for direction in directions {
        location = move_santa(location, *direction);
        visited.insert(location);
    }

    Ok(visited.len())
}

fn part2(directions: &[CardinalDirection]) -> Result<usize> {
    let mut visited = HashSet::new();
    let mut locations = [IPoint2D::ORIGIN, IPoint2D::ORIGIN];
    visited.insert(locations[0]);
    for (pos, direction) in directions.iter().enumerate() {
        let ix = pos % 2;
        locations[ix] = move_santa(locations[ix], *direction);
        visited.insert(locations[ix]);
    }

    Ok(visited.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        assert_eq!(part1(&parse_chars(">")?)?, 2);
        assert_eq!(part1(&parse_chars("^>v<")?)?, 4);
        assert_eq!(part1(&parse_chars("^v^v^v^v^v")?)?, 2);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        assert_eq!(part2(&parse_chars("^v")?)?, 3);
        assert_eq!(part2(&parse_chars("^>v<")?)?, 3);
        assert_eq!(part2(&parse_chars("^v^v^v^v^v")?)?, 11);

        Ok(())
    }
}
