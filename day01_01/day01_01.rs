use std::env;
use std::fs;
use std::path::Path;

fn get_two_digit_number(s: &str) -> u64 {
    // n.b., spec says first and last digit, but it does not say that it cannot be the same
    // string character position.
    let binding: Vec<_> = s.trim_matches(|c: char| !char::is_ascii_digit(&c)).chars().collect();
    let number: u64 = format!("{}{}",&binding[0], &binding[binding.len()-1]).parse().unwrap();
    return number;
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
