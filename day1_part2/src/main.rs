// Here is the source: https://adventofcode.com/2023/day/1#part2
// Here is the sample input: https://adventofcode.com/2023/day/1/input

use std::fs;

fn main() {
    // Read the file
    let lines: Vec<String> = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    // Iterate over each line
    // Gather number from each line
    // Sum all the lines and output the sum

    let mut sum = 0;
    for line in lines {
        sum += calculate_calibration(line.as_str());
    }

    println!("output is {}", sum);
}

fn calculate_calibration(input: &str) -> u32 {
    let temp = convert_text_digit_to_numbers(input);
    let input = temp.as_str();
    // Iterate over the string
    // Deteremine if each character is a numerical
    // Store the first one in tens (default is 100)
    // Store the last one in units (default is 100)
    // At the end of the iteration if units is 100 then assign tens to units
    // return tens * 10 + units

    // The default value here is assumed to be 100 because tens and units can only be single digit numbers
    const DEFAULT_VALUE: u32 = 100;
    let mut tens: u32 = DEFAULT_VALUE;
    let mut units: u32 = DEFAULT_VALUE;

    for c in input.chars() {
        if !c.is_numeric() {
            continue;
        }

        let num = c.to_digit(10).unwrap();

        if tens == DEFAULT_VALUE {
            tens = num;
            continue;
        }

        units = num;
    }

    if tens == DEFAULT_VALUE {
        panic!("Didn't find any number in the string: {}", input);
    }

    if units == DEFAULT_VALUE {
        units = tens
    }

    tens * 10 + units
}

fn convert_text_digit_to_numbers(input: &str) -> String {
    input
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_calibration() {
        // Define a list of strings containing input and respective output
        let inputs = [
            ("two1nine", 29),
            ("eightwothree", 83),
            ("abcone2threexyz", 13),
            ("xtwone3four", 24),
            ("4nineeightseven2", 42),
            ("zoneight234", 14),
            ("7pqrstsixteen", 76),
            ("479", 49),
            ("5klxgb1", 51),
            ("zthree4", 34),
            ("3nine4fourjclspd152rpv", 32),
            ("9four7", 97),
            ("threeonetwotwo2", 32),
            ("v6two", 62),
            ("4l4", 44),
            ("sixone9", 69),
            // ("",),
        ];

        for i in inputs.iter() {
            assert_eq!(calculate_calibration(i.0), i.1);
        }
    }

    #[test]
    fn test_convert_text_digit_to_numbers() {
        // Define a list of strings containing input and respective output
        let inputs = [
            ("two1nine", "two2two1nine9nine"),
            ("eightwothree", "eight8eightwo2twothree3three"),
            ("abcone2threexyz", "abcone1one2three3threexyz"),
            ("xtwone3four", "xtwo2twone1one3four4four"),
            ("4nineeightseven2", "4nine9nineeight8eightseven7seven2"),
            ("zoneight234", "zone1oneight8eight234"),
            ("7pqrstsixteen", "7pqrstsix6sixteen"),
            ("479", "479"),
            ("5klxgb1", "5klxgb1"),
            ("zthree4", "zthree3three4"),
            ("3nine4fourjclspd152rpv", "3nine9nine4four4fourjclspd152rpv"),
        ];

        for i in inputs.iter() {
            assert_eq!(convert_text_digit_to_numbers(i.0), i.1);
        }
    }
}
