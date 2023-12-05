use std::collections::HashSet;
use std::fs;

const INPUT_FILE: &str = "input";
// const INPUT_FILE: &str = "smallinput";

fn main() {
    let lines: Vec<String> = fs::read_to_string(INPUT_FILE)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut points: u32 = 0;
    for line in lines.iter() {
        let (h1, h2) = get_hash_sets(line);
        let power = h1.intersection(&h2).collect::<Vec<&u32>>().len();
        if power == 0 {
            continue;
        }
        let power = power - 1;

        points += 2_u32.pow(power.try_into().unwrap());
    }

    println!("{}", points);
}

fn get_hash_sets(s: &str) -> (HashSet<u32>, HashSet<u32>) {
    // line -> Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    let card_number_and_numbers: Vec<&str> = s.split(":").collect();
    // card_number_and_numbers -> ["Card 1", "41 48 83 86 17 | 83 86  6 31 17  9 48 53"]
    let numbers: Vec<&str> = card_number_and_numbers[1].trim().split("|").collect();
    // numbers -> ["41 48 83 86 17", "83 86  6 31 17  9 48 53"]

    // winning_numbers -> [41, 48, 83, 86, 17]
    let winning_numbers: Vec<u32> = numbers[0]
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .filter_map(|x| {
            if *x == "" {
                None
            } else {
                Some(x.parse::<u32>().unwrap())
            }
        })
        .collect();

    // [83, 86, 6, 31, 17, 9, 48, 53]
    let numbers_you_have: Vec<u32> = numbers[1]
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .filter_map(|x| {
            // some of the entries could be like: ... "86", "", "6", ...
            if *x == "" {
                None
            } else {
                Some(x.parse::<u32>().unwrap())
            }
        })
        .collect();

    (
        HashSet::from_iter(winning_numbers),
        HashSet::from_iter(numbers_you_have),
    )
}
