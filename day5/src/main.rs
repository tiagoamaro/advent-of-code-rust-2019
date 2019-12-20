fn parameter_value(parameter: i32, mode: usize, commands: &Vec<i32>) -> i32 {
    match mode {
        0 => commands[parameter as usize],
        1 => parameter,
        _ => panic!("Mode does not exist"),
    }
}

fn main() {
    let input = std::fs::read_to_string("src/input").unwrap();
    let mut input_text = String::new();
    let commands_string: Vec<&str> = input.trim().split(",").collect();
    let mut commands: Vec<i32> = commands_string
        .into_iter()
        .map(|str| str.parse::<i32>().unwrap())
        .collect();

    let mut index: usize = 0;
    while commands[index] != 99 {
        let instruction = format!("{:0>5}", commands[index].to_string());

        let opcode = format!(
            "{}{}",
            instruction.chars().nth(3).unwrap(),
            instruction.chars().nth(4).unwrap()
        )
        .parse::<i32>()
        .unwrap();
        let first_mode = format!("{}", instruction.chars().nth(2).unwrap())
            .parse::<usize>()
            .unwrap();
        let second_mode = format!("{}", instruction.chars().nth(1).unwrap())
            .parse::<usize>()
            .unwrap();
        let third_mode = format!("{}", instruction.chars().nth(0).unwrap())
            .parse::<usize>()
            .unwrap();

        // println!("opcode: {}", opcode);
        // println!("first mode: {}", first_mode);
        // println!("second mode: {}", second_mode);
        // println!("third mode: {}", third_mode);
        // println!("---------");

        match opcode {
            1 => {
                commands[index + 3] = parameter_value(commands[index + 1], first_mode, &commands)
                    + parameter_value(commands[index + 2], second_mode, &commands);
                index += 3;
            }
            2 => {
                commands[index + 3] = parameter_value(commands[index + 1], first_mode, &commands)
                    * parameter_value(commands[index + 2], second_mode, &commands);
                index += 3;
            }
            3 => {
                let target_position = commands[index + 1];

                println!("Input: ");
                std::io::stdin()
                    .read_line(&mut input_text)
                    .expect("failed to read from stdin");
                commands[target_position as usize] = input_text.trim().parse::<i32>().unwrap();
                println!("{:?}", commands);
                index += 2;
            }
            4 => {
                let target_position = commands[index + 1];

                println!("Output: {:?}", commands[target_position as usize]);
                index += 2;
            }
            99 => {}
            _ => panic!("Unknown opcode"),
        }
    }

    println!("{:?}", commands);
}
