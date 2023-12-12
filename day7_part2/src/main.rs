// Here is the source: https://adventofcode.com/2023/day/7
// Here is the sample input: https://adventofcode.com/2023/day/7/input
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

const INPUT_FILE: &str = "input";
// const INPUT_FILE: &str = "smallinput";

fn main() {
    let lines: Vec<String> = fs::read_to_string(INPUT_FILE)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let inp = Input::new(lines);
    // Create buckets based on HandType
    let mut bucket: HashMap<HandType, Vec<Hand>> = HashMap::new();
    for entry in inp.entries.iter() {
        let v = bucket.entry(entry.category).or_insert_with(|| vec![]);
        v.push(entry.clone());
    }

    let hand_types = vec![
        HandType::HighCard,
        HandType::OnePair,
        HandType::TwoPair,
        HandType::ThreeOfAKind,
        HandType::FullHouse,
        HandType::FourOfAKind,
        HandType::FiveOfAKind,
    ];

    let mut sorted_hands: Vec<Hand> = Vec::new();
    for ht in hand_types.iter() {
        match bucket.get_mut(&ht) {
            None => continue,
            Some(c) => {
                // let c = bucket.get_mut(ht).unwrap();
                c.sort_by(|a, b| compare_strings(a, b));
                sorted_hands.append(c);
            }
        };
    }

    let mut winnings: u32 = 0;
    for (idx, sh) in sorted_hands.iter().enumerate() {
        winnings += (u32::try_from(idx).unwrap() + 1) * sh.bid;
    }

    println!("{:?}", winnings);
}

fn compare_strings(a: &Hand, b: &Hand) -> Ordering {
    let cards: HashMap<char, u32> = HashMap::from([
        ('A', 12),
        ('K', 11),
        ('Q', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
        ('J', 0),
    ]);

    for it in a.hand.chars().zip(b.hand.chars()) {
        let (a_chr, b_chr) = it;

        if cards[&a_chr] > cards[&b_chr] {
            return Ordering::Greater;
        } else if cards[&a_chr] < cards[&b_chr] {
            return Ordering::Less;
        }
    }

    Ordering::Equal
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, Clone)]
struct Hand {
    hand: String,
    bid: u32,
    category: HandType,
}

#[derive(Debug)]
struct Input {
    entries: Vec<Hand>,
}

impl Input {
    fn new(lines: Vec<String>) -> Self {
        let mut ret = Input { entries: vec![] };
        for line in lines.iter() {
            // Split the line on space
            let hand_and_bid: Vec<&str> = line.split(" ").collect();
            ret.entries.push(Hand {
                hand: hand_and_bid[0].to_string(),
                bid: hand_and_bid[1].parse::<u32>().unwrap(),
                category: kind_of_hand(hand_and_bid[0].to_string()),
            });
        }

        ret
    }
}

fn kind_of_hand(s: String) -> HandType {
    let mut count: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        count.entry(c).or_insert_with(|| s.matches(c).count());
    }

    let contains_j = s.contains('J');

    match count.len() {
        1 => return HandType::FiveOfAKind,
        5 => {
            match count.get(&'J') {
                // If there is one J that means we can form one pair
                Some(v) => {
                    if *v == 1 {
                        return HandType::OnePair;
                    }
                }
                _ => {}
            }

            return HandType::HighCard;
        }
        2 => {
            for (_, v) in count.iter() {
                if !contains_j && (*v == 1 || *v == 4) {
                    return HandType::FourOfAKind;
                }

                if !contains_j && (*v == 3 || *v == 2) {
                    return HandType::FullHouse;
                }

                return HandType::FiveOfAKind;
            }
        }
        3 => {
            for (_, v) in count.iter() {
                if !contains_j {
                    if *v == 3 {
                        return HandType::ThreeOfAKind;
                    }

                    if *v == 2 {
                        return HandType::TwoPair;
                    }

                    continue;
                }

                match count.get(&'J') {
                    Some(j_count) => {
                        if *j_count == 3 && *v == 1 {
                            return HandType::FourOfAKind;
                        }

                        if *j_count == 2 && *v == 2 {
                            return HandType::FourOfAKind;
                        }

                        if *j_count == 1 && *v == 3 {
                            return HandType::FourOfAKind;
                        }

                        if *j_count == 1 && *v == 2 {
                            return HandType::FullHouse;
                        }
                    }
                    None => {}
                }
            }
        }
        4 => {
            if !contains_j {
                return HandType::OnePair;
            }

            return HandType::ThreeOfAKind;
        }
        _ => panic!("unknown size of hand"),
    }

    HandType::OnePair
}
