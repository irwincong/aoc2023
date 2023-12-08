use std::env;
use std::fs;
use std::path::Path;

fn get_two_digit_number(s: &str) -> u64 {
    let s_mod = String::from(s).replace("one", "one1one").replace("two", "two2two").replace("three", "three3three").replace("four", "four4four").replace("five", "five5five").replace("six", "six6six").replace("seven", "seven7seven").replace("eight", "eight8eight").replace("nine", "nine9nine");
    let binding = s_mod.chars().collect::<Vec<char>>();
    let digits: Vec<_> = binding.iter().filter(|c| c.is_digit(10)).map(|c| c.to_digit(10)).collect();
    assert!(digits.len() >= 1, "{:?} does not have enough digits", digits);
    // n.b., spec says first and last digit, but it does not say that it cannot be the same
    // string character position.
    let tens: u32 = digits.first().unwrap().unwrap();
    let ones: u32 = digits.last().unwrap().unwrap();
    let two_digit_number: u64 = (tens * 10 + ones).into();

    println!("{} - {} - {}", s, s_mod, two_digit_number);
    // println!("{} - {}", s, two_digit_number);
    return two_digit_number;
}

// This is the main function.
fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1, "Usage: <bin> <input>");
    let input_path = Path::new(&args[1]);

    // open input file and read line by line get two digit value, then sum them up
    let ans: u64 = fs::read_to_string(input_path).unwrap().lines().map(|line| get_two_digit_number(line)).sum();
    println!("{}", ans);
}
