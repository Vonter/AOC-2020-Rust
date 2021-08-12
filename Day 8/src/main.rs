use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn execute_instruction(instruction: &str, accumulator: &mut Vec<i32>, eip: usize) -> usize {
    let operation: &str = instruction.split_whitespace().next().unwrap();
    let operand: &str = instruction.split_whitespace().next().unwrap();

    match operation {
        "acc" => accumulator.push(accumulator.last().unwrap() + operand.parse::<i32>().unwrap()),
        "jmp" => {
            accumulator.push(*accumulator.last().unwrap());
            return (eip as i32 + operand.parse::<i32>().unwrap()) as usize;
        }
        &_ => {
            accumulator.push(*accumulator.last().unwrap());
            return eip + 1;
        }
    }

    eip + 1
}

fn fuzz_instruction(instruction: &str, eip: usize) -> usize {
    let operation: &str = instruction.split_whitespace().next().unwrap();
    let operand: &str = instruction.split_whitespace().next().unwrap();

    match operation {
        "nop" => {
            if eip as i32 + operand.parse::<i32>().unwrap() > 0 {
                (eip as i32 + operand.parse::<i32>().unwrap()) as usize
            } else {
                0_usize
            }
        }
        &_ => eip + 1,
    }
}

fn execute_program(
    lines: Vec<String>,
    program_size: usize,
    instruction_cache: &mut Vec<usize>,
    eip: &mut usize,
    acc: &mut Vec<i32>,
) -> i32 {
    loop {
        if *eip >= program_size {
            println!("Execution Done!");
            println!("ACC: {}", acc.last().unwrap());
            return *acc.last().unwrap();
        }
        if instruction_cache.contains(eip) {
            return *acc.last().unwrap();
        } else {
            instruction_cache.push(*eip);
            *eip = execute_instruction(&lines[*eip], acc, *eip);
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("numbers.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let program_size: usize = lines.len();

    let mut instruction_cache: Vec<usize> = Vec::new();

    let mut eip: usize = 0;
    let mut acc: Vec<i32> = vec![0];

    execute_program(
        lines.clone(),
        program_size,
        &mut instruction_cache,
        &mut eip,
        &mut acc,
    );
    println!("Stuck in a Loop!");
    println!("ACC: {}", acc.last().unwrap());
    loop {
        if eip >= program_size {
            break;
        }
        eip = fuzz_instruction(
            &lines[*instruction_cache.last().unwrap()],
            *instruction_cache.last().unwrap(),
        );
        instruction_cache.pop();
        acc.pop();
        execute_program(
            lines.clone(),
            program_size,
            &mut instruction_cache.clone(),
            &mut eip,
            &mut acc.clone(),
        );
    }

    Ok(())
}
