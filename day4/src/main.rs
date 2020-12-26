extern crate regex;

use std::fs;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
struct Passport {
    byr: Option<String>, // Birth Year
    iyr: Option<String>, // Issue Year
    eyr: Option<String>, // Expiration Year
    hgt: Option<String>, // Height
    hcl: Option<String>, // Hair Color
    ecl: Option<String>, // Eye Color
    pid: Option<String>, // Passport ID
    cid: Option<String>, // Country ID
}

impl Passport {
    fn parse(input: &str) -> Passport {
        let split = input
            .trim()
            .split(|c| c == ' ' || c == '\n')
            .map(|keyval| {
                let mut s = keyval.split(":");
                (s.next().unwrap(), s.next().unwrap())
            });
        let mut map: HashMap<String, String> = HashMap::new();
        for (key, val) in split {
            map.insert(key.to_string(), val.to_string());
        }

        Passport {
            byr: map.get("byr").map_or_else(|| None, |v| Some(v.clone())),
            iyr: map.get("iyr").map_or_else(|| None, |v| Some(v.clone())),
            eyr: map.get("eyr").map_or_else(|| None, |v| Some(v.clone())),
            hgt: map.get("hgt").map_or_else(|| None, |v| Some(v.clone())),
            hcl: map.get("hcl").map_or_else(|| None, |v| Some(v.clone())),
            ecl: map.get("ecl").map_or_else(|| None, |v| Some(v.clone())),
            pid: map.get("pid").map_or_else(|| None, |v| Some(v.clone())),
            cid: map.get("cid").map_or_else(|| None, |v| Some(v.clone())),
        }
    }

    fn is_valid(&self) -> bool {
        self.byr != None &&
        self.iyr != None &&
        self.eyr != None &&
        self.hgt != None &&
        self.hcl != None &&
        self.ecl != None &&
        self.pid != None
    }

    fn is_valid_ex(&self) -> bool {
        let validate_num_between = |s: &String, low, high| {
            match s.parse::<i32>() {
                Ok(num) => num >= low && num <= high,
                Err(_) => false,
            }
        };

        let validate_byr = |byr| {
            validate_num_between(byr, 1920, 2002)
        };

        let validate_iyr = |iyr| {
            validate_num_between(iyr, 2010, 2020)
        };

        let validate_eyr = |eyr| {
            validate_num_between(eyr, 2020, 2030)
        };

        let validate_hgt = |hgt: &String| {
            if hgt.ends_with("in") {
                match hgt.strip_suffix("in").unwrap().parse::<i32>() {
                    Ok(num) => num >= 50 && num <= 76,
                    Err(_) => false
                }
            } else if hgt.ends_with("cm") {
                match hgt.strip_suffix("cm").unwrap().parse::<i32>() {
                    Ok(num) => num >= 150 && num <= 193,
                    Err(_) => false
                }
            } else {
                false
            }
        };

        let validate_hcl = |hcl: &String| {
            if hcl.starts_with("#") {
                let s = hcl.strip_prefix("#").unwrap();
                if s.len() == 6 {
                    let regex = Regex::new(r"[0-9a-fA-F]{6}").unwrap();
                    let result = regex.find(s);
                    result != None
                } else {
                    false
                }
            } else {
                false
            }
        };

        let validate_ecl = |ecl: &String| {
            match ecl.as_str() {
                "amb" => true,
                "blu" => true,
                "brn" => true,
                "gry" => true,
                "grn" => true,
                "hzl" => true,
                "oth" => true,
                _ => false
            }
        };

        let validate_pid = |pid: &String| {
            if pid.len() == 9 {
                let regex = Regex::new(r"[0-9]{9}").unwrap();
                let result = regex.find(pid);
                result != None
            } else {
                false
            }
        };


        self.is_valid() &&
        validate_byr(&self.byr.as_ref().unwrap()) &&
        validate_iyr(&self.iyr.as_ref().unwrap()) &&
        validate_eyr(&self.eyr.as_ref().unwrap()) &&
        validate_hgt(&self.hgt.as_ref().unwrap()) &&
        validate_hcl(&self.hcl.as_ref().unwrap()) &&
        validate_ecl(&self.ecl.as_ref().unwrap()) &&
        validate_pid(&self.pid.as_ref().unwrap())
    }
}

fn main() {
    let contents = fs::read_to_string("day4.txt")
        .expect("something went wrong reading day4.txt");

    let valid_passports = contents
        .split("\n\n")
        .map(|s| Passport::parse(&s))
        .fold(0, |acc, elem| {
            if elem.is_valid_ex() {
                acc + 1
            } else {
                acc
            }
        });
    println!("valid passports: {}", valid_passports);
    // for s in passports {
    //     let passport = Passport::parse(&s);
    //     println!("passport: {:?}", passport);
    // }
}

