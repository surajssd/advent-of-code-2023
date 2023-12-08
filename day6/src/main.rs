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
    let times: Vec<u32> = time_str[1]
        .split(" ")
        .filter_map(|x| {
            if x == "" {
                None
            } else {
                Some(x.parse::<u32>().unwrap())
            }
        })
        .collect();
    // line 2 => distance
    let distance_str: Vec<&str> = lines[1].split(":").collect();
    let distances: Vec<u32> = distance_str[1]
        .split(" ")
        .filter_map(|x| {
            if x == "" {
                None
            } else {
                Some(x.parse::<u32>().unwrap())
            }
        })
        .collect();

    let mut output: u32 = 1;
    for idx in 0..distances.len() {
        let mut combo: u32 = 0;
        let time = times[idx];
        let record_distance = distances[idx];

        for ht in 1..time {
            let speed = ht;
            let rem_time = time - ht;
            let distance = rem_time * speed;
            if distance > record_distance {
                combo += 1;
            }
        }

        output *= combo;
    }

    println!("{}", output);
}
