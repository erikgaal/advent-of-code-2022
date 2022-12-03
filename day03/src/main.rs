type Item = char;

type BackpackWithCompartments = (Vec<char>, Vec<char>);

#[derive(Clone)]
struct Backpack {
    items: Vec<Item>,
}

impl Backpack {
    fn split(self) -> (Vec<Item>, Vec<Item>) {
        let (a, b) = self.items.split_at(self.items.len() / 2);
        return (a.to_vec(), b.to_vec());
    }
}

impl From<&str> for Backpack {
    fn from(item: &str) -> Self {
        Backpack {
            items: item.chars().collect(),
        }
    }
}

fn parse(s: &str) -> BackpackWithCompartments {
    return Backpack::from(s).split();
}

fn item_priority(item: Item) -> Result<i32, (Item, i32)> {
    let ord = item as i32;
    return match ord {
        97..=123 => Ok(ord - 96),
        65..=91  => Ok(ord - 64 + 26),
        _        => Err((item, ord))
    };
}

fn items_intersect(a: Vec<Item>, b: Vec<Item>) -> Vec<Item> {
    let mut result: Vec<Item> = a.iter().filter(|char| b.contains(char)).copied().collect();

    result.dedup();

    return result;
}

fn backpack_priority(backpack: BackpackWithCompartments) -> i32 {
    return items_intersect(backpack.0, backpack.1).iter().map(|c| item_priority(*c).unwrap()).sum();
}

fn sum_priorities(s: &str) -> i32 {
    return s.lines()
        .map(|s| parse(s))
        .map(|b| backpack_priority(b))
        .sum()
}

fn find_group_item(group: Vec<Backpack>) -> Item {
    return group.iter()
        .map(|b| b.items.clone())
        .reduce(|acc, item| items_intersect(acc, item))
        .unwrap()
        .first()
        .unwrap()
        .clone();
}

fn sum_group_item_priorities(s: &str) -> i32 {
    return s.lines()
        .map(|b| Backpack::from(b))
        .collect::<Vec<Backpack>>()
        .chunks(3)
        .map(|group| find_group_item(group.to_vec()))
        .map(|i| item_priority(i).unwrap())
        .sum::<i32>();
}

fn main() {
    println!(
        "Part One: {}",
        sum_priorities(include_str!("../input.txt")),
    );

    println!(
        "Part Two: {}",
        sum_group_item_priorities(include_str!("../input.txt")),
    )
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(("vJrwpWtwJgWr".chars().collect(), "hcsFMMfFFhFp".chars().collect()), Backpack::from("vJrwpWtwJgWrhcsFMMfFFhFp").split())
    }

    #[test]
    fn test_item_priority() {
        assert_eq!(1, item_priority('a').unwrap());
        assert_eq!(26, item_priority('z').unwrap());
        assert_eq!(27, item_priority('A').unwrap());
        assert_eq!(52, item_priority('Z').unwrap());
    }

    #[test]
    fn test_items_intersect() {
        assert_eq!(vec!['p'], items_intersect("vJrwpWtwJgWr".chars().collect(), "hcsFMMfFFhFp".chars().collect()));
        assert_eq!(vec!['L'], items_intersect("jqHRNqRjqzjGDLGL".chars().collect(), "rsFMfFZSrLrFZsSL".chars().collect()));
        assert_eq!(vec!['P'], items_intersect("PmmdzqPrV".chars().collect(), "vPwwTWBwg".chars().collect()));
        assert_eq!(vec!['v'], items_intersect("wMqvLMZHhHMvwLH".chars().collect(), "jbvcjnnSBnvTQFn".chars().collect()));
        assert_eq!(vec!['t'], items_intersect("ttgJtRGJ".chars().collect(), "QctTZtZT".chars().collect()));
        assert_eq!(vec!['s'], items_intersect("CrZsJsPPZsGz".chars().collect(), "wwsLwLmpwMDw".chars().collect()));
    }

    #[test]
    fn test_backpack_priority() {
        assert_eq!(16, backpack_priority(parse("vJrwpWtwJgWrhcsFMMfFFhFp")));
        assert_eq!(38, backpack_priority(parse("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")));
        assert_eq!(42, backpack_priority(parse("PmmdzqPrVvPwwTWBwg")));
        assert_eq!(22, backpack_priority(parse("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn")));
        assert_eq!(20, backpack_priority(parse("ttgJtRGJQctTZtZT")));
        assert_eq!(19, backpack_priority(parse("CrZsJsPPZsGzwwsLwLmpwMDw")));
    }

    #[test]
    fn test_sum_priorities() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(157, sum_priorities(input));
    }

    #[test]
    fn test_find_group_item() {
        assert_eq!('r', find_group_item("vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg".lines().map(|b| Backpack::from(b)).collect()));

        assert_eq!('Z', find_group_item("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw".lines().map(|b| Backpack::from(b)).collect()));
    }
}