const INPUT_END: usize = 815432;
const INPUT_START: usize = 272091;
const PASSWORD_LENGTH: usize = 6;

fn obeys_criteria(password: usize) -> bool {
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

fn main() {
    let part_one_result = (INPUT_START..INPUT_END)
        .map(|password| obeys_criteria(password))
        .filter(|obeys| obeys == &true)
        .collect::<Vec<bool>>()
        .len();

    println!("{:?}", part_one_result);
}
