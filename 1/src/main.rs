use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn find_digits(input: &str) -> (Option<char>, Option<char>) {
    let digit_words = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut first_digit = None;
    let mut last_digit = None;
    let mut first_digit_index = usize::MAX;
    let mut last_digit_index = 0;

    for (i, c) in input.char_indices() {
        if c.is_digit(10) {
            if first_digit.is_none() {
                first_digit = Some(c);
                first_digit_index = i;
            }
            last_digit = Some(c);
            last_digit_index = i;
        }
    }

    for (i, word) in digit_words.iter().enumerate() {
        if let Some(index) = input.find(*word) {
            if index < first_digit_index {
                first_digit = Some(char::from_digit(i as u32, 10).unwrap());
                first_digit_index = index;
            }
        }

        if let Some(index) = input.rfind(*word) {
            if index > last_digit_index {
                last_digit = Some(char::from_digit(i as u32, 10).unwrap());
                last_digit_index = index;
            }
        }
    }

    (first_digit, last_digit)
}


fn main() {
    println!("Advent of Code 1");

    let path = Path::new("input.txt");
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open input: {}", why),
        Ok(file) => file,
    };

    let mut code: u32 = 0;
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read input: {}", why),
        Ok(_) => {
            for line in s.split("\n") {
                if !line.is_empty() {
                    let (first, last) = find_digits(line);
                    let chars = first.unwrap().to_string() + &last.unwrap().to_string();
                    code += chars.parse::<u32>().unwrap();
                    println!("Line: {} | Chars: {} | Code: {}", line, chars, code);
                }
                
            }
            println!("Final Code: {}", code);
        }
    }
}