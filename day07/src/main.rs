use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

fn main() {
    let text = include_str!("../input.txt");

    let root = Dir::build(text.lines().map(|l| l.to_string()));
    let total_size = root.borrow().size();
    println!("Total size: {}", total_size);
    println!("Part 1: {}", root.borrow().size_if_under_100_000());
    let space_needed = total_size - 40_000_000;
    let mut all_sizes: Vec<usize> = root.borrow().get_all_sizes().into_iter().filter(|x| *x > space_needed).collect();
    all_sizes.sort();
    println!("Part 2: {}", all_sizes[0]);
}

#[derive(Debug)]
struct Dir {
    name: String,
    subdirs: HashMap<String, Rc<RefCell<Self>>>,
    file_sizes: Vec<usize>
}

impl Dir {
    pub fn new(name: &str) -> Self {
        Dir {
            name: name.to_string(),
            subdirs: HashMap::new(),
            file_sizes: Vec::new()
        }
    }

    pub fn build(input: impl Iterator<Item = String>) -> Rc<RefCell<Self>> {
        let root = Rc::new(RefCell::new(Dir::new("/")));
        let mut cwd = vec![Rc::clone(&root)];

        for line in input {
            let str = line else {
                continue;
            };

            let tokens: Vec<&str> = str.split_whitespace().collect();

            match tokens[0] {
                "$" => {
                    match tokens[1] {
                        "cd" => {
                            match tokens[2] {
                                ".." => { cwd.pop(); },
                                "/" => {
                                    cwd.truncate(1) },
                                dir @ _ => {
                                    let new = Rc::clone(cwd.last().unwrap().borrow_mut().subdirs.get(dir).unwrap());
                                    cwd.push(new);
                                }
                            }
                        },
                        "ls" => {},  // Don't have to do anything special here
                        _ => {}
                    }
                }
                "dir" => {
                    cwd.last_mut().unwrap().borrow_mut().subdirs.insert(String::from(tokens[1]), Rc::new(RefCell::new(Dir::new(tokens[1]))));
                },
                size @ _ => {
                    cwd.last_mut().unwrap().borrow_mut().file_sizes.push(size.parse::<usize>().unwrap_or(0));
                }  // It's a file
            }
        }

        root
    }

    fn size(&self) -> usize {
        self.file_sizes.iter().sum::<usize>() + self.subdirs.iter().map(|(_, dir)| dir.borrow().size()).sum::<usize>()
    }

    fn size_if_under_100_000(&self) -> usize {
        let actual_size = self.size();
        let size_below_me = self.subdirs.iter().map(|(_, dir)| dir.borrow().size_if_under_100_000()).sum::<usize>();

        if actual_size <= 100_000 {
            actual_size + size_below_me
        } else {
            size_below_me
        }
    }

    fn get_all_sizes(&self) -> Vec<usize> {
        let mut result = vec![self.size()];
        for (_, dir) in &self.subdirs {
            result.extend(dir.borrow().get_all_sizes());
        }
        result
    }
}