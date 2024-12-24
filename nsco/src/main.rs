use std::collections::HashSet;
use std::env;

// Validate if the base is within the acceptable range (2 to 36)
fn validate_base(base: u8) -> bool {
    (2..=36).contains(&base)
}

// Validate if the number is valid for the given base
fn validate_number(num: &str, base: u8) -> bool {
    let valid_chars: HashSet<char> = (0..base)
        .map(|i| if i < 10 {
            (b'0' + i) as char
        } else {
            (b'a' + (i - 10)) as char
        })
        .flat_map(|c| vec![c, c.to_ascii_uppercase()])
        .collect();

    num.chars().all(|c| valid_chars.contains(&c))
}

// Convert a number from one base to another
fn convert_base(number: &str, base_from: u8, base_to: u8) -> String {
    // Convert the input number to decimal
    let decimal_value = number
        .chars()
        .fold(0u32, |acc, char| {
            let digit_value = if char.is_ascii_digit() {
                char as u32 - b'0' as u32
            } else {
                char.to_ascii_lowercase() as u32 - b'a' as u32 + 10
            };
            acc * base_from as u32 + digit_value
        });

    // Convert the decimal integer to the target base
    let mut result = String::new();
    let mut value = decimal_value;
    while value > 0 {
        let digit = value % base_to as u32;
        let digit_char = if digit < 10 {
            (b'0' + digit as u8) as char
        } else {
            (b'A' + (digit as u8 - 10)) as char
        };
        result.push(digit_char);
        value /= base_to as u32;
    }

    // Return "0" for zero input or reverse the result string
    if result.is_empty() {
        "0".to_string()
    } else {
        result.chars().rev().collect()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Ensure correct number of arguments
    if args.len() != 4 {
        eprintln!("Usage: nsco <number> <base_from> <base_to>");
        return;
    }

    let num = &args[1];
    let base_from: u8 = match args[2].parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Error: Base from which you are converting must be an integer.");
            return;
        }
    };
    let base_to: u8 = match args[3].parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Error: Base to which you are converting must be an integer.");
            return;
        }
    };

    // Validate bases
    if !validate_base(base_from) {
        eprintln!("Error: Base from which you are converting must be between 2 and 36.");
        return;
    }
    if !validate_base(base_to) {
        eprintln!("Error: Base to which you are converting must be between 2 and 36.");
        return;
    }

    // Validate number
    if !validate_number(num, base_from) {
        eprintln!(
            "Error: The number '{}' contains invalid characters for base {}.",
            num, base_from
        );
        return;
    }

    let result = convert_base(num, base_from, base_to);
    println!(
        "Number: {}, Base from: {}, Base to: {}, Result: {}",
        num, base_from, base_to, result
    );
}