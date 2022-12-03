type Item = char;

type Backpack = (Vec<char>, Vec<char>);

pub fn parse(s: &str) -> Backpack {
    let (first, second) = s.split_at(s.len() / 2);
    return (first.chars().collect(), second.chars().collect());
}

fn item_priority(item: Item) -> Result<i32, (Item, i32)> {
    let ord = item as i32;
    return match ord {
        97..=123 => Ok(ord - 96),
        65..=91  => Ok(ord - 64 + 26),
        _        => Err((item, ord))
    };
}

fn items_intersect(a: Vec<char>, b: Vec<char>) -> Vec<char> {
    let mut result: Vec<char> = a.iter().filter(|char| b.contains(char)).copied().collect();

    result.dedup();

    return result;
}

fn backpack_priority(backpack: Backpack) -> i32 {
    return items_intersect(backpack.0, backpack.1).iter().map(|c| item_priority(*c).unwrap()).sum();
}

fn sum_priorities(s: &str) -> i32 {
    return s.lines()
        .map(|s| parse(s))
        .map(|b| backpack_priority(b))
        .sum()
}

fn main() {
    println!(
        "Part One: {}",
        sum_priorities(include_str!("../input.txt")),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(("vJrwpWtwJgWr".chars().collect(), "hcsFMMfFFhFp".chars().collect()), parse("vJrwpWtwJgWrhcsFMMfFFhFp"))
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
}