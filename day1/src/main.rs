use std::fs;

#[allow(dead_code)]
fn day1(contents: &String) {
    let mut result: u32 = 0;
    for line1 in contents.lines() {
        let num1 = line1.parse::<u32>().unwrap();
        for line2 in contents.lines() {
            let num2 = line2.parse::<u32>().unwrap();
            let sum = num1 + num2;
            if sum == 2020 {
                 result = num1 * num2;
            }
        }
    }

    println!("result: {}", result);
}

#[allow(dead_code)]
fn day1x(contents: &String) {
    let mut result: u32 = 0;
    for line1 in contents.lines() {
        let num1 = line1.parse::<u32>().unwrap();
        for line2 in contents.lines() {
            let num2 = line2.parse::<u32>().unwrap();
            for line3 in contents.lines() {
                let num3 = line3.parse::<u32>().unwrap();
                let sum = num1 + num2 + num3;
                if sum == 2020 {
                    result = num1 * num2 * num3;
                }
            }
        }
    }

    println!("result: {}", result);
}

fn main() {
    let contents = fs::read_to_string("day1.txt")
        .expect("something went wrong reading day1.txt");

    day1x(&contents);
}
