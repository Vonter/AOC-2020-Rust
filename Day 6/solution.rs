use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn count_answers_in_response (response: &Vec<String>) -> (i32, i32) {

    let mut answers:HashMap<char, i32> = HashMap::new();
    let mut everyone_answered: i32 = 0;

    for line in response.iter() {
        for field in line.chars() {
            if !answers.contains_key(&field) {
                answers.insert(field, 0);
            }
            *answers.get_mut(&field).unwrap() += 1;
        }
    }

    for (_key, value) in &answers {
        if *value == response.len() as i32 {
            everyone_answered += 1;
        }
    }

    return (answers.len() as i32, everyone_answered);
}

fn main() -> io::Result<()> {

    let file = File::open("numbers.txt")?;
    let reader = BufReader::new(file);
    let mut response: Vec<String> = Vec::new();
    let mut count_result: (i32, i32);
    let mut sum_of_anyone_answer_count: i32 = 0;
    let mut sum_of_everyone_answer_count: i32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            count_result = count_answers_in_response(&response);
            sum_of_anyone_answer_count += count_result.0;
            sum_of_everyone_answer_count += count_result.1;
            response.clear();
            continue;
        }
        response.push(line);
    }

    count_result = count_answers_in_response(&response);
    sum_of_anyone_answer_count += count_result.0;
    sum_of_everyone_answer_count += count_result.1;

    println!("Part 1: {}", sum_of_anyone_answer_count);
    println!("Part 2: {}", sum_of_everyone_answer_count);

    Ok(())
}
