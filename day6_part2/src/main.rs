// Here is the source: https://adventofcode.com/2023/day/6
// Here is the sample input: https://adventofcode.com/2023/day/6/input
use std::fs;

const INPUT_FILE: &str = "input";
// const INPUT_FILE: &str = "smallinput";

fn main() {
    let lines: Vec<String> = fs::read_to_string(INPUT_FILE)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    // line 1 => time
    let time_str: Vec<&str> = lines[0].split(":").collect();
    let time = time_str[1].replace(" ", "").parse::<u64>().unwrap();

    // line 2 => distance
    let distance_str: Vec<&str> = lines[1].split(":").collect();
    let distance = distance_str[1].replace(" ", "").parse::<u64>().unwrap();

    let mut output: u64 = 1;
    let mut combo: u64 = 0;

    for ht in 1..time {
        let speed = ht;
        let rem_time = time - ht;
        let d = rem_time * speed;
        if d > distance {
            combo += 1;
        }
    }

    output *= combo;

    println!("{}", output);
}
