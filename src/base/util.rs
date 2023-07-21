use core::fmt;
use std::slice::Iter;

pub type Coordinate = (i32, i32, i32);

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

impl Direction {
    pub fn to_int(&self) -> i32 {
        match self {
            Direction::North => 0,
            Direction::South => 1,
            Direction::East => 2,
            Direction::West => 3,
            Direction::Up => 4,
            Direction::Down => 5,
        }
    }

    pub fn from_int(int: i32) -> Direction {
        match int {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            3 => Direction::West,
            4 => Direction::Up,
            5 => Direction::Down,
            _ => panic!("Invalid direction int"),
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    pub fn iterator() -> Iter<'static, Direction> {
        [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
            Direction::Up,
            Direction::Down,
        ]
        .iter()
    }
}

impl fmt::Display for Direction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let dir_str = match self {
			Direction::North => "North",
			Direction::South => "South",
			Direction::East => "East",
			Direction::West => "West",
			Direction::Up => "Up",
			Direction::Down => "Down",
		};
		write!(f, "{}", dir_str)
	}
}

/// Steps a coordinate in a specified direction.
pub fn step_in_dir(coord: Coordinate, dir: Direction) -> Coordinate {
    let (x, y, z) = coord;
    match dir {
        Direction::North => (x, y + 1, z),
        Direction::South => (x, y - 1, z),
        Direction::East => (x + 1, y, z),
        Direction::West => (x - 1, y, z),
        Direction::Up => (x, y, z + 1),
        Direction::Down => (x, y, z - 1),
    }
}

pub fn get_neighbors(coord: Coordinate) -> Vec<Coordinate> {
    let mut neighbors = Vec::new();
    for dir in [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
        Direction::Up,
        Direction::Down,
    ]
    .iter()
    {
        neighbors.push(step_in_dir(coord, *dir));
    }
    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    //could be broken down into multiple tests, but I'm lazy
    #[test]
    fn test_step_in_dir() {
        assert_eq!(step_in_dir((0, 0, 0), Direction::North), (0, 1, 0));
        assert_eq!(step_in_dir((0, 0, 0), Direction::South), (0, -1, 0));
        assert_eq!(step_in_dir((0, 0, 0), Direction::East), (1, 0, 0));
        assert_eq!(step_in_dir((0, 0, 0), Direction::West), (-1, 0, 0));
        assert_eq!(step_in_dir((0, 0, 0), Direction::Up), (0, 0, 1));
        assert_eq!(step_in_dir((0, 0, 0), Direction::Down), (0, 0, -1));
    }
}
