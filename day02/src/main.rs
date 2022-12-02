use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn value(self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn score(self, _rhs: Self) -> i32 {
        if self.beats(_rhs) {
            return self.value() + 6
        } else if _rhs.beats(self) {
            return self.value()
        } else {
            return self.value() + 3
        }
    }

    fn beats(self, _rhs: Self) -> bool {
        return self == Self::Rock && _rhs == Self::Scissors
         || self == Self::Paper && _rhs == Self::Rock
         || self == Self::Scissors && _rhs == Self::Paper
    }

    fn move_for_result(self, result: GameResult) -> Move {
        match result {
            GameResult::Draw => self,
            GameResult::Win => self.win(),
            GameResult::Lose => self.lose(),
        }
    }

    fn lose(self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }
    
    fn win(self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Move, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _         => Err(()),
        }
    }
}

pub enum GameResult {
    Win,
    Lose,
    Draw,
}

impl FromStr for GameResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _   => Err(())
        }
    }
}

fn parse(input: &str) -> Vec<(&str, &str)> {
    return input.lines()
        .map(|l| l.split(' '))
        .map(|mut v| (v.next().unwrap(), v.next().unwrap()))
        .collect();
}

fn part1(input: &str) -> i32 {
    return parse(input)
        .iter()
        .map(|(a, b)| (Move::from_str(a).unwrap(), Move::from_str(b).unwrap()))
        .map(|(a, b)| b.score(a))
        .sum::<i32>();
}

fn part2(input: &str) -> i32 {
    return parse(input)
        .iter()
        .map(|(a, b)| (Move::from_str(a).unwrap(), GameResult::from_str(b).unwrap()))
        .map(|(a, b)| a.move_for_result(b).score(a))
        .sum::<i32>()
}

fn main() {
    println!(
        "Part One: {}",
        part1(include_str!("../input.txt"))
    );

    println!(
        "Part Two: {}",
        part2(include_str!("../input.txt"))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beats() {
        assert!(Move::Rock.beats(Move::Scissors));
        assert!(Move::Scissors.beats(Move::Paper));
        assert!(Move::Paper.beats(Move::Rock));
    }

    #[test]
    fn test_score() {
        assert_eq!(8, Move::Paper.score(Move::Rock))
    }

    #[test]
    fn test_part1() {
        assert_eq!(15, part1("A Y\nB X\nC Z"))
    }

    #[test]
    fn test_part2() {
        assert_eq!(12, part2("A Y\nB X\nC Z"))
    }
}
