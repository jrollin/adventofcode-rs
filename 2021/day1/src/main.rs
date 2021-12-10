use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let count = part_one(&contents);
    println!("part1: {}", count);

    let count = part_bis(&contents);
    println!("part2: {}", count);
}

fn part_one(contents: &str) -> usize {
    let lines = contents
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim())
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let nb = lines.windows(2).filter(|pair| pair[0] < pair[1]).count();
    dbg!(&nb);
    nb
}

fn part_bis(contents: &str) -> usize {
    let lines = contents
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim())
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let ter = lines
        .windows(3)
        .map(|t| t.iter().sum())
        .collect::<Vec<i32>>();
    let nb = ter.windows(2).filter(|pair| pair[0] < pair[1]).count();
    nb
}

#[cfg(test)]
mod test {

    use super::*;
    const INPUT: &str = r"
        199 
        200 
        208 
        210 
        200 
        207 
        240 
        269 
        260 
        263 
    ";

    #[test]
    fn one() {
        assert_eq!(part_one(INPUT), 7)
    }

    #[test]
    fn two() {
        assert_eq!(part_bis(INPUT), 5)
    }
}
