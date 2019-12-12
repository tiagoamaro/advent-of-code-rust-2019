const INPUT_END: usize = 815432;
const INPUT_START: usize = 272091;
const PASSWORD_LENGTH: usize = 6;

fn obeys_criteria_part_one(password: usize) -> bool {
    let password_string = password.to_string();
    let chars: Vec<char> = password_string.chars().into_iter().collect();

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
    let password_string = password.to_string();
    let chars: Vec<char> = password_string.chars().into_iter().collect();

    let adjacent_digits = (1..PASSWORD_LENGTH)
        .map(|index| vec![chars[index - 1], chars[index]])
        .any(|chars| chars[0] == chars[1]);

    let adjacent_digits = (1..PASSWORD_LENGTH-1)
        .map(|index| vec![chars[index - 1], chars[index], chars[index+1]])
        .any(|chars| chars[0] == chars[1] && chars[1] == chars[2]);

    println!("{:?}", adjacent_digits);

    let mut crescent_digits = true;
    let mut max = chars[0];
    for character in chars {
        max = vec![character, max].into_iter().max().unwrap();
        if character < max {
            crescent_digits = false;
            break;
        }
    }

    false && crescent_digits
}

fn main() {
    // let part_one_result = (INPUT_START..INPUT_END)
    //     .map(|password| obeys_criteria_part_one(password))
    //     .filter(|obeys| obeys == &true)
    //     .collect::<Vec<bool>>()
    //     .len();

    // println!("{:?}", part_one_result);

    // let part_two_result = (INPUT_START..INPUT_END)
    //     .map(|password| obeys_criteria_part_two(password))
    //     .filter(|obeys| obeys == &true)
    //     .collect::<Vec<bool>>()
    //     .len();

    // println!("{:?}", part_one_result);

    println!("{:?}", obeys_criteria_part_two(112233));
    println!("{:?}", obeys_criteria_part_two(123444));
    println!("{:?}", obeys_criteria_part_two(111122));
}
