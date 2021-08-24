use std::io;

fn check_in_border(index: i32, delta: i32, col: i32) -> bool {
    !(delta == -col - 1 && index % col != 0
        || delta == -col
        || delta == -col + 1 && index % col != (col - 1)
        || delta == -1 && index % col != 0
        || delta == 1 && index % col != (col - 1)
        || delta == col + 1 && index % col != (col - 1)
        || delta == col
        || delta == col - 1 && index % col != 0)
}

fn get_element(seats: &[u8], index: i32, delta: i32, col: i32, part2: bool) -> u8 {
    let border = check_in_border(index, delta, col);
    let index = index + delta;

    if index < 0 || index >= seats.len() as i32 || border {
        return 0;
    }

    let border = check_in_border(index, delta, col);
    if seats[index as usize] == 8 || seats[index as usize] == 1 {
        return seats[index as usize];
    } else if part2 && !border {
        return get_element(seats, index, delta, col, part2);
    }

    0
}

fn get_adjacent(seats: &[u8], index: i32, col: i32, part2: bool) -> u8 {
    let mut sum: u8 = 0;
    sum += get_element(seats, index, -col - 1, col, part2);
    sum += get_element(seats, index, -col, col, part2);
    sum += get_element(seats, index, -col + 1, col, part2);
    sum += get_element(seats, index, -1, col, part2);
    sum += get_element(seats, index, 1, col, part2);
    sum += get_element(seats, index, col - 1, col, part2);
    sum += get_element(seats, index, col, col, part2);
    sum += get_element(seats, index, col + 1, col, part2);
    sum
}

fn update_seats(seats: &mut Vec<u8>, col: i32, part2: bool) -> bool {
    let mut updated_seats: Vec<u8> = Vec::new();
    for index in 0..seats.len() {
        if seats[index] == 0 {
            updated_seats.push(0);
            continue;
        }
        let sum = get_adjacent(seats, index as i32, col, part2);

        if (!part2 && sum > 31) || (part2 && sum > 39) {
            updated_seats.push(1);
        } else if sum < 9 {
            updated_seats.push(8);
        } else {
            updated_seats.push(seats[index]);
        }
    }
    let change = seats != &updated_seats;
    seats.clone_from(&updated_seats);
    change
}

fn main() -> io::Result<()> {
    let lines = include_str!("numbers.txt");
    let mut seats: Vec<u8> = Vec::new();

    for character in lines.chars() {
        match character {
            'L' => seats.push(1),
            '\n' => continue,
            _ => seats.push(0),
        }
    }

    let columns: i32 = lines.split('\n').next().unwrap().len() as i32;
    let mut seats_backup: Vec<u8> = seats.clone();

    while update_seats(&mut seats, columns, false) {}
    println!(
        "Part 1 Count: {}",
        seats.iter().filter(|c| **c == 8_u8).count()
    );

    while update_seats(&mut seats_backup, columns, true) {}
    println!(
        "Part 2 Count: {}",
        seats_backup.iter().filter(|c| **c == 8_u8).count()
    );

    Ok(())
}
