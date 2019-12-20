use std::collections::HashMap;

const INPUT_END: usize = 815432;
const INPUT_START: usize = 272091;
const PASSWORD_LENGTH: usize = 6;

fn obeys_criteria_part_one(password: usize) -> bool {
    let chars: Vec<char> = password.to_string().chars().into_iter().collect();

    let adjacent_digits = (1..PASSWORD_LENGTH)
        .map(|index| vec![chars[index - 1], chars[index]])
        .any(|chars| chars[0] == chars[1]);

    let mut crescent_digits = true;
    let mut max = chars[0];
    for character in chars {
        max = vec![character, max].into_iter().max().unwrap();
        if character < max {
            crescent_digits = false;
            break;
        }
    }

    adjacent_digits && crescent_digits
}

fn obeys_criteria_part_two(password: usize) -> bool {
    let chars: Vec<char> = password.to_string().chars().into_iter().collect();

    let mut crescent_digits = true;
    let mut max = &chars[0];
    for character in &chars {
        max = vec![character, max].into_iter().max().unwrap();
        if character < max {
            crescent_digits = false;
            break;
        }
    }

    let mut adjacent_map: HashMap<char, usize> = HashMap::new();
    for index in 1..PASSWORD_LENGTH {
        let first_char = chars[index - 1];
        let second_char = chars[index];

        if first_char == second_char {
            match adjacent_map.get_mut(&first_char) {
                Some(count) => {
                    *count += 1;
                }
                None => {
                    adjacent_map.insert(first_char, 1);
                }
            }
        }
    }
    let adjacent_numbers = adjacent_map.into_iter().any(|(_number, count)| count == 1);

    crescent_digits && adjacent_numbers
}

fn main() {
    let part_one_result = (INPUT_START..INPUT_END)
        .map(|password| obeys_criteria_part_one(password))
        .filter(|obeys| obeys == &true)
        .collect::<Vec<bool>>()
        .len();

    println!("{:?}", part_one_result);

    let part_two_result = (INPUT_START..INPUT_END)
        .map(|password| obeys_criteria_part_two(password))
        .filter(|obeys| obeys == &true)
        .collect::<Vec<bool>>()
        .len();

    println!("{:?}", part_two_result);
}
