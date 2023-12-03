// Here is the source: https://adventofcode.com/2023/day/1
// Here is the sample input: https://adventofcode.com/2023/day/1/input

use std::fs;
use std::str::FromStr;

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

    return tens * 10 + units;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_calibration() {
        // Define a list of strings containing input and respective output
        let inputs = [
            ("sq5fivetwothree1", 51),
            ("six5gc", 55),
            ("1abc2", 12),
            ("pqr3stu8vwx", 38),
            ("a1b2c3d4e5f", 15),
            ("treb7uchet", 77),
        ];

        for i in inputs.iter() {
            assert_eq!(calculate_calibration(i.0), i.1);
        }
    }
}
