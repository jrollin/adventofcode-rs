use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Invalid file provided");
    let count = part_one(&content);
    println!("part1: {}", count)
}

fn part_one(content: &str) -> usize {
    let lines: Vec<Vec<char>> = content
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect();
    let total = *&lines.len();
    let average = total / 2;
    let first = &lines.iter().nth(0).unwrap();

    let mut gamma = String::new();
    let mut epsilon = String::new();
    for i in 0..first.len() {
        let count = &lines.iter().filter(|l| l[i] == '1').count();
        if count > &average {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let gamma = u64::from_str_radix(&gamma, 2).unwrap();
    let epsilon = u64::from_str_radix(&epsilon, 2).unwrap();

    (gamma * epsilon) as usize
}

#[cfg(test)]
mod test {

    use super::*;
    const INPUT: &str = r"
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    ";

    #[test]
    fn one() {
        assert_eq!(part_one(INPUT), 198)
    }
}
