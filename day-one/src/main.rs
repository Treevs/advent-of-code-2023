use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


fn main() {
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let digits: Vec<_> = line.chars().filter(|c| c.is_digit(10)).collect();

        if digits.len() > 0 {
            if let (Some(&first_char), Some(&last_char)) = (digits.first(), digits.last()) {
                if let (Some(first_digit), Some(last_digit)) = (first_char.to_digit(10), last_char.to_digit(10)) {
                    let two_digit_number = first_digit * 10 + last_digit;
                    sum += two_digit_number;
                }
            }
        }
    }

    println!("The sum of all two-digit numbers is: {}", sum);
}
