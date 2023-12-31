advent_of_code::solution!(1);

use regex::{Captures, Regex};

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"(\d{1})").unwrap();
    let numbers_collected: Vec<u32> = input.lines().map(
        |x| {
            let numbers: Vec<&str> = re.find_iter(x).map(|y| y.as_str()).collect();
            let first = numbers.first().unwrap();
            let last = numbers.last().unwrap();
            let concat_number = "".to_owned() + first + last;
            let parsed_number: u32 = concat_number.parse::<u32>().ok().unwrap();
            parsed_number
        }
    ).collect();
    // for each line
        // filter all non numbers
            // get first and last number
                // create a number
    // Sum all numbers
    let summed_number: u32 = numbers_collected.iter().sum();
    Some(summed_number)
}

pub fn part_two(input: &str) -> Option<u32> {
    let preprocess_re = Regex::new("(zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let preprocessed_reult = preprocess_re.replace_all(input, |cap: &Captures| {
        match &cap[0] {
            "zero" => "0",
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => panic!("We should never have gotten here!"),
        }.to_string()
    });
    let re = Regex::new(r"(\d{1})").unwrap();
    let numbers_collected: Vec<u32> = preprocessed_reult.lines().map(
        |x| {
            let numbers: Vec<&str> = re.find_iter(x).map(|y| y.as_str()).collect();
            let first = numbers.first().unwrap();
            let last = numbers.last().unwrap();
            let concat_number = "".to_owned() + first + last;
            let parsed_number: u32 = concat_number.parse::<u32>().ok().unwrap();
            parsed_number
        }
    ).collect();
    // for each line
    // filter all non numbers
    // get first and last number
    // create a number
    // Sum all numbers
    let summed_number: u32 = numbers_collected.iter().sum();
    Some(summed_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
