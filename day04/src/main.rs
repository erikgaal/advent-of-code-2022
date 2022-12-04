ay use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Pair {
    left: Assignment,
    right: Assignment,
}

impl Pair {
    fn contains(&self) -> bool {
        return self.left.contains(self.right) || self.right.contains(self.left);
    }
}

impl From<&str> for Pair {
    fn from(s: &str) -> Self {
        let assignments: Vec<Assignment> = s.split(',').map(|s| Assignment::from(s)).collect();
        let slice = assignments.as_slice();

        return match slice {
            [left, right] => Pair {
                left: *left,
                right: *right,
            },
            _ => Pair {
                left: Assignment { assignment: (0, 0) },
                right: Assignment { assignment: (0, 0) },
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Assignment {
    assignment: (i32, i32),
}

impl Assignment {
    fn contains(&self, other: Self) -> bool {
        let (start, end) = self.assignment;
        let (other_start, other_end) = other.assignment;

        return start <= other_start && end >= other_end;
    }
}

impl From<(i32, i32)> for Assignment {
    fn from((start, stop): (i32, i32)) -> Self {
        return Assignment {
            assignment: (start, stop)
        }
    }
}

impl From<&str> for Assignment {
    fn from(s: &str) -> Self {
        let assignment: Vec<i32> = s.split('-').map(|s| FromStr::from_str(s).unwrap()).collect();
        let slice = assignment.as_slice();

        return match slice {
            [start, stop] => Assignment { assignment: (*start, *stop) },
            _ => Assignment { assignment: (0, 0) },
        }
    }
}

fn how_many_assignment_pairs_fully_contain_the_other(pairs: Vec<Pair>) -> i32 {
    return pairs.iter().filter(|p| p.contains()).count() as i32;
}

fn main() {
    let input = include_str!("../input.txt");
    let pairs = input.lines().map(|s| Pair::from(s));

    println!(
        "Part One: {}",
        how_many_assignment_pairs_fully_contain_the_other(pairs.collect()),
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(Pair { left: Assignment{assignment: (2,4) }, right: Assignment { assignment: (6,8) } }, Pair::from("2-4,6-8"));
        assert_eq!(Pair { left: Assignment{assignment: (2,3) }, right: Assignment { assignment: (4,5) } }, Pair::from("2-3,4-5"));
    }

    #[test]
    fn test_pair_contains() {
        assert_eq!(false, Pair::from("2-4,6-8").contains());
        assert_eq!(false, Pair::from("5-7,7-9").contains());
        assert_eq!(true, Pair::from("2-8,3-7").contains());

        // .678.
        // 567..
        assert_eq!(false, Pair::from("16-38,15-37").contains());
        assert_eq!(false, Pair::from("3-3,4-91").contains());
    }

    #[test]
    fn test_how_many_assignment_pairs_fully_contain_the_other() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let pairs = input.lines().map(|s| Pair::from(s));

        assert_eq!(2, how_many_assignment_pairs_fully_contain_the_other(pairs.collect()));
    }
}