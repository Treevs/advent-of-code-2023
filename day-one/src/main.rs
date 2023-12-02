use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


fn main() {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;
    let number_words: HashMap<&str, char> = [
        ("one", '1'), ("two", '2'), ("three", '3'),
        ("four", '4'), ("five", '5'), ("six", '6'), ("seven", '7'),
        ("eight", '8'), ("nine", '9')
    ].iter().cloned().collect();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut first_digit = None;
        let mut last_digit = None;
        let mut current_word = String::new();

        for c in line.chars() {
            if first_digit.is_some() {
                break;
            }
            if c.is_digit(10) {
                first_digit = Some(c);
                break;
            } else if c.is_alphabetic() {
                current_word.push(c);
                let mut found_substring = false;
                for start in 0..current_word.len() {
                    for len in 1..=current_word.len() - start {
                        let substring = &current_word[start..start + len];
                        if let Some(&digit) = number_words.get(substring) {
                            first_digit = Some(digit);
                            found_substring = true;
                            break;
                        }
                    }
                    if found_substring {
                        break;
                    }
                }
            }
        }
        current_word.clear();

        for c in line.chars().rev() {
            if last_digit.is_some() {
                break;
            }
            if c.is_digit(10) {
                last_digit = Some(c);
                break;
            } else {
                current_word.insert_str(0, &c.to_string());
                let mut found_substring = false;
                for start in 0..current_word.len() {
                    for len in 1..=current_word.len() - start {
                        let substring = &current_word[start..start + len];
                        if let Some(&digit) = number_words.get(substring) {
                            last_digit = Some(digit);
                            found_substring = true;
                            break;
                        }
                    }
                    if found_substring {
                        break;
                    }
                }
            }
        }
        if let (Some(first_digit), Some(last_digit)) = (first_digit, last_digit) {
            if let (Some(first_digit), Some(last_digit)) = (first_digit.to_digit(10), last_digit.to_digit(10)) {
                let two_digit_number = first_digit * 10 + last_digit;
                sum += two_digit_number;
            }
        }
    }
    
    println!("The sum of all two-digit numbers is: {}", sum);
}
