#[derive(Debug, PartialEq)]
struct Program {
    instructions: Vec<Instruction>
}

union Instruction {
    addx: AddInstruction,
    noop: NoopInstruction,
}

impl TryFrom<&str> for Instruction {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts = s.split_ascii_whitespace()
            .collect::<Vec<&str>>();

        match *parts.first().unwrap() {
            "addx" => Ok(Instruction {
                addx: AddInstruction {
                value: 0,
            }}),
            "noop" => Ok(Instruction {
                noop: NoopInstruction {}
            }),
            _      => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct NoopInstruction {

}

#[derive(Debug, Clone, Copy)]
struct AddInstruction {
    value: i32,
}

impl From<&str> for Program {
    fn from(string: &str) -> Self {
        Program {
            instructions: string.lines()
                .map(|l| Instruction::try_from(l).unwrap())
                .collect()
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!("../example1.txt");

        assert_eq!(Program {
            instructions: vec![
                Instruction {
                    noop: NoopInstruction {  }
                },
                Instruction {
                    addx: AddInstruction { value: 3 }
                },
                Instruction {
                    addx: AddInstruction { value: -5 }
                },
            ]
        }, Program::from(input));
    }
}

