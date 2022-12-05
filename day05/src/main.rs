use regex::Regex;

#[derive(Debug, PartialEq)]
struct Supply {
    stacks: Vec<Vec<Crate>>,
}

type Crate = char;

impl Supply {
    fn move_crates(&mut self, amount: i32, from: usize, to: usize) {
        for _i in 0..amount {
            let carry = self.stacks.get_mut(from - 1).unwrap().pop().unwrap();
            self.stacks.get_mut(to - 1).unwrap().push(carry);
        }
    }

    fn message(&self) -> String {
        return self.stacks.iter()
            .map(|stack| stack.last().unwrap())
            .collect();
    }

    fn apply(&mut self, procedure: Procedure) {
        for (amount, from, to) in procedure.steps {
            self.move_crates(amount, from, to);
        }
    }
}

impl From<&str> for Supply {
    fn from(input: &str) -> Self {
        const COLUMN_WIDTH: usize = 4;
        let n_columns = (input.lines().next().unwrap().len() as f32 / COLUMN_WIDTH as f32).round() as usize;

        let mut stacks: Vec<Vec<Crate>> = vec![vec![]; n_columns];

        input.lines()
            .rev() // Start from the bottom
            .skip(1) // Skip the last line, it's the column numbers.
            .for_each(|line| {
                line.chars()
                    .enumerate()
                    .filter(|&(i, _)| i % COLUMN_WIDTH == 1)
                    .map(|(_, c)| c)
                    .enumerate()
                    .for_each(|(i, c)| {
                        if c != ' ' {
                            stacks.get_mut(i).unwrap().push(c);
                        }
                    });
            });

        return Supply {
            stacks: stacks,
        }
    }
}

#[derive(Debug)]
struct Procedure {
    steps: Vec<Step>,
}

type Step = (i32, usize, usize);

impl From<&str> for Procedure {
    fn from(input: &str) -> Self {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

        return Procedure {
            steps: re.captures_iter(input)
                .map(|cap| (
                    cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    cap.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                    cap.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                ))
                .collect(),
        };
    }
}

fn parse(input: &str) -> (Supply, Procedure) {
    let split = input.split("\n\n").collect::<Vec<&str>>();

    return (
        Supply::from(split[0]),
        Procedure::from(split[1]),
    )
}

fn main() {
    let (mut supply, procedure) = parse(include_str!("../input.txt"));

    supply.apply(procedure);

    println!(
        "Part One: {}",
        supply.message(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_crates() {
        let mut supply = Supply {
            stacks: vec![
                vec!['Z', 'N'],
                vec!['M', 'C', 'D'],
                vec!['P'],
            ],
        };

        supply.move_crates(1, 2, 1);
        assert_eq!(vec![
            vec!['Z', 'N', 'D'],
            vec!['M', 'C'],
            vec!['P'],
        ], supply.stacks);

        supply.move_crates(3, 1, 3);
        assert_eq!(vec![
            vec![],
            vec!['M', 'C'],
            vec!['P', 'D', 'N', 'Z'],
        ], supply.stacks);

        supply.move_crates(2, 2, 1);
        assert_eq!(vec![
            vec!['C', 'M'],
            vec![],
            vec!['P', 'D', 'N', 'Z'],
        ], supply.stacks);

        supply.move_crates(1, 1, 2);
        assert_eq!(vec![
            vec!['C'],
            vec!['M'],
            vec!['P', 'D', 'N', 'Z'],
        ], supply.stacks);

        assert_eq!("CMZ", supply.message());
    }

    #[test]
    fn test_supply_from_str() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ";
        let supply = Supply::from(input);

        assert_eq!(vec![
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P'],
        ], supply.stacks)
    }

    #[test]
    fn test_procedure_from_str() {
        let input = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let procedure = Procedure::from(input);

        assert_eq!(vec![
            (1, 2, 1),
            (3, 1, 3),
            (2, 2, 1),
            (1, 1, 2),
        ], procedure.steps)
    }
}