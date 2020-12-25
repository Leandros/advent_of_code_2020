use std::fs;

struct PasswordPolicy {
    letter: String,
    low: i32,
    high: i32,
}

impl PasswordPolicy {
    fn parse(policy: &str) -> PasswordPolicy {
        let mut policy_parts = policy.split(" ");
        let policy_range = policy_parts.next().unwrap();
        let policy_letter = policy_parts.next().unwrap();
        let mut policy_range_split = policy_range.split("-");
        let policy_range_low = policy_range_split.next().unwrap().parse::<i32>().unwrap();
        let policy_range_high = policy_range_split.next().unwrap().parse::<i32>().unwrap();

        PasswordPolicy {
            letter: policy_letter.to_string(),
            low: policy_range_low,
            high: policy_range_high,
        }
    }
}

fn get_char(chars: &Vec<char>, index: usize) -> Option<char> {
    if chars.len() >= index {
        Some(chars[index])
    } else {
        None
    }
}

#[allow(dead_code)]
fn day2(contents: &String) {
    let mut valid = 0;
    for line in contents.lines() {
        let mut parts = line.split(":");

        let policy_string = parts.next().unwrap();
        let policy = PasswordPolicy::parse(&policy_string);
        let password = parts.next().unwrap().trim();

        let count = password.matches(&policy.letter).count() as i32;
        if count >= policy.low && count <= policy.high {
            valid = valid + 1
        }
    }

    println!("valid passwords: {}", valid);
}

#[allow(dead_code)]
fn day2x(contents: &String) {
    let mut valid = 0;
    for line in contents.lines() {
        let mut parts = line.split(":");

        let policy_string = parts.next().unwrap();
        let policy = PasswordPolicy::parse(&policy_string);
        let password = parts.next().unwrap().trim();
        let letter = policy.letter.chars().collect::<Vec<char>>()[0];

        let chars = password.chars().collect::<Vec<char>>();
        let char1 = get_char(&chars, (policy.low - 1) as usize);
        let char2 = get_char(&chars, (policy.high - 1) as usize);
        if (char1 == Some(letter) && (char2 != Some(letter)))
          || ((char1 != Some(letter)) && char2 == Some(letter)) {
            println!("password '{}' is valid (policy: {}/{} '{}')", password, policy.low, policy.high, policy.letter);
            valid = valid + 1;
        }
    }

    println!("valid passwords: {}", valid);
}

fn main() {
    let contents = fs::read_to_string("day2.txt")
        .expect("something went wrong reading day2.txt");

    // day2(&contents);
    day2x(&contents);
}
