use std::fs;

const STEP: usize = 4;

fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    let commands_string: Vec<&str> = input.trim().split(",").collect();
    let mut commands: Vec<u32> = commands_string
        .into_iter()
        .map(|str| str.parse::<u32>().unwrap())
        .collect();

    let mut index: usize = 0;
    while commands[index] != 99 {
        let first_position = commands[index + 1] as usize;
        let second_position = commands[index + 2] as usize;
        let target_position = commands[index + 3] as usize;

        let first_value = commands[first_position];
        let second_value = commands[second_position];

        match commands[index] {
            1 => commands[target_position] = first_value + second_value,
            2 => commands[target_position] = first_value * second_value,
            99 => {}
            _ => println!("error"),
        }

        index += STEP;
    }

    println!("{:?}", commands)
}
