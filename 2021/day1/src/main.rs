use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    part_one(&contents);
    part_bis(&contents);
}

fn part_one(contents: &String) {
    let lines = contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim())
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let iter = lines.windows(2).filter(|pair| pair[0] < pair[1]).count();
    dbg!(&iter);
    ();
}

fn part_bis(contents: &String) {
    let lines = contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim())
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let ter = lines
        .windows(3)
        .map(|t| t.iter().sum())
        .collect::<Vec<i32>>();
    let iter = ter.windows(2).filter(|pair| pair[0] < pair[1]).count();
    dbg!(&iter);
    ();
}
