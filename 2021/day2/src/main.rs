use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let count = part_one(&contents);
    println!("part1: {}", count);

    let count = part_bis(&contents);
    println!("part2: {}", count);
}

fn part_one(contents: &str) -> usize {
    let lines: Vec<(&str, i32)> = contents
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let pair: Vec<&str> = line.split_whitespace().collect();
            (pair[0], pair[1].parse::<i32>().unwrap())
        })
        .collect();

    struct Position {
        horizontal: i32,
        depth: i32,
    }

    let mut position = Position {
        horizontal: 0,
        depth: 0,
    };

    for line in lines {
        match line {
            ("forward", pos) => position.horizontal += pos,
            ("down", pos) => position.depth += pos,
            ("up", pos) => position.depth -= pos,
            (_, _) => (),
        }
    }
    (position.depth * position.horizontal) as usize
}

fn part_bis(contents: &str) -> usize {
    let lines: Vec<(&str, i32)> = contents
        .trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let pair: Vec<&str> = line.split_whitespace().collect();
            (pair[0], pair[1].parse::<i32>().unwrap())
        })
        .collect();

    struct Position {
        horizontal: i32,
        depth: i32,
        aim: i32,
    }

    let mut position = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    for line in lines {
        match line {
            ("forward", pos) => {
                position.horizontal += pos;
                position.depth += pos * position.aim
            }
            ("down", pos) => position.aim += pos,
            ("up", pos) => position.aim -= pos,
            (_, _) => (),
        }
    }

    (position.horizontal * position.depth) as usize
}

#[cfg(test)]
mod test {

    use super::*;
    const INPUT: &str = r"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    ";

    #[test]
    fn one() {
        assert_eq!(part_one(INPUT), 150)
    }

    #[test]
    fn two() {
        assert_eq!(part_bis(INPUT), 900)
    }
}
