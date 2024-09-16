#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<&str> for Direction {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _   => Err(()),
        }
    }
}

fn main() {
    println!("Hello, world!");
}

fn parse_procedure(input: &str) -> Vec<(Direction, i32)>
{
    return input.lines()
        .map(|l| l.split(' ').collect::<Vec<&str>>())
        .map(|s| (Direction::try_from(s[0]).unwrap(), s[1].parse::<i32>().unwrap()))
        .collect::<Vec<(Direction, i32)>>()
}

fn run_procedure(input: &str)

fn visited_positions(input: &str) -> Vec<(usize, usize)> {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visited_positions()
    {
        let input = include_str!("../example.txt");

        assert_eq!(13, visited_positions(input));
    }

    #[test]
    fn test_parse_procedure()
    {
        let input = include_str!("../example.txt");

        assert_eq!(vec![
            (Direction::Right, 4),
            (Direction::Up, 4),
            (Direction::Left, 3),
            (Direction::Down, 1),
            (Direction::Right, 4),
            (Direction::Down, 1),
            (Direction::Left, 5),
            (Direction::Right, 2),
        ], parse_procedure(input));
    }
}