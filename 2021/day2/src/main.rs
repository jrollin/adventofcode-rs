use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    part_one(&contents);
    part_bis(&contents);
}

fn part_one(contents: &String) {
    let lines: Vec<(&str, i32)> = contents
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
    dbg!(position.depth * position.horizontal);

    ();
}

fn part_bis(contents: &String) {
    let lines: Vec<(&str, i32)> = contents
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

    dbg!(position.horizontal * position.depth);
    ()
}
