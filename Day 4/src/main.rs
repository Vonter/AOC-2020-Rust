use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

fn valid_passport (passport: &Vec<String>) -> bool {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut fields: Vec<String> = Vec::new();
    let mut values: Vec<String> = Vec::new();

    for line in passport.iter() {
        for field in line.split_whitespace() {
            fields.push(field.split(':').nth(0).unwrap().to_string());
            values.push(field.split(':').nth(1).unwrap().to_string());
        }
    }
    for req_field in required_fields.iter() {
        if !fields.contains(&req_field.to_string()) {
            return false;
        }
    }

    let valid_eye_color = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let re_hcl = Regex::new(r"#[0-9a-f]{6}$").unwrap();
    let re_hgt_cm = Regex::new(r"^1([5-8][0-9]|9[0-3])cm").unwrap();
    let re_hgt_in = Regex::new(r"^([5-6][0-9]|7[0-6])in").unwrap();

    for (index, field) in fields.iter().enumerate() {
        if field == "byr" && values[index].parse::<i32>().unwrap() >= 1910 && values[index].parse::<i32>().unwrap() <= 2002 {
            continue
        } else if field == "iyr" && values[index].parse::<i32>().unwrap() >= 2010 && values[index].parse::<i32>().unwrap() <= 2020 {
            continue
        } else if field == "eyr" && values[index].parse::<i32>().unwrap() >= 2020 && values[index].parse::<i32>().unwrap() <= 2030 {
            continue
        } else if field == "hgt" && (re_hgt_cm.is_match(values[index].as_str()) || re_hgt_in.is_match(values[index].as_str())) {
            continue
        } else if field == "hcl" && re_hcl.is_match(values[index].as_str()) {
            continue
        } else if field == "ecl" && valid_eye_color.contains(&values[index].as_str()) {
            continue
        } else if field == "pid" && values[index].len() == 9 {
            continue
        } else if field == "cid" {
            continue
        } else {
            return false;
        }
    }

    return true;
}

fn main() -> io::Result<()> {

    let file = File::open("numbers.txt")?;
    let reader = BufReader::new(file);
    let mut passport: Vec<String> = Vec::new();
    let mut valid_passport_count: i32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            if valid_passport(&passport) {
                valid_passport_count = valid_passport_count + 1;
            }
            passport.clear();
            continue;
        }
        passport.push(line);
    }

    if valid_passport(&passport) {
        valid_passport_count = valid_passport_count + 1;
    }

    println!("{}", valid_passport_count);

    Ok(())
}
