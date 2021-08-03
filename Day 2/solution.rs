use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {

    let file = File::open("numbers.txt")?;
    let reader = BufReader::new(file);
    let mut valid_count: i32 = 0;
    let mut part_two_valid_count: i32 = 0;

    for line in reader.lines() {

        let mut counter: i32 = 0;
        let line = line.unwrap();

        let policy_number1 = line.split_whitespace().nth(0).unwrap()
                                .split("-").nth(0).unwrap()
                                .parse::<i32>().unwrap();
        let policy_number2 = line.split_whitespace().nth(0).unwrap()
                                .split("-").nth(1).unwrap()
                                .parse::<i32>().unwrap();
        let policy_char = line.split_whitespace().nth(1).unwrap().
                                split(":").nth(0).unwrap()
                                .parse::<char>().unwrap();
        let password = line.split_whitespace().nth(2).unwrap();
        
        for character in password.chars() {
            if character == policy_char {
                counter = counter + 1;
            }
        }

        if counter >= policy_number1 && counter <= policy_number2 {
            valid_count = valid_count + 1;
        }

        part_two_valid_count = part_two_valid_count + 
                                (((password.chars().nth((policy_number1 - 1) as usize).unwrap() == policy_char) ^
                                (password.chars().nth((policy_number2 - 1) as usize).unwrap() == policy_char)) as i32);
    }

    println!("Part 1: {}", valid_count);
    println!("Part 2: {}", part_two_valid_count);

    Ok(())
}
