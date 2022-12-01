fn sorted(vec: Vec<i32>) -> Vec<i32> {
    let mut copy = vec.to_vec();
    copy.sort();
    return copy;
}

fn reversed(vec: Vec<i32>) -> Vec<i32> {
    let mut copy = vec.to_vec();
    copy.reverse();
    return copy;
}

fn main() {
    println!(
        "Part One: {}",
        include_str!("../input.txt")
            .split("\n\n")
            .map(|n| n.lines().map(|n| n.parse::<i32>().unwrap()).sum::<i32>())
            .max().expect("No answer!")
    );

    println!(
        "Part Two: {}",
        reversed(sorted(include_str!("../input.txt")
            .split("\n\n")
            .map(|n| n.lines().map(|n| n.parse::<i32>().unwrap()).sum::<i32>())
            .collect::<Vec<i32>>()))
            .iter()
            .take(3)
            .sum::<i32>()
    );
}
