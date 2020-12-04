use regex::Regex;
use crate::common::read_lines;

fn is_valid(passport: &str) -> bool {
    let byr = Regex::new(r"byr:(.)+").unwrap();
    let iyr = Regex::new(r"iyr:(.)+").unwrap();
    let eyr = Regex::new(r"eyr:(.)+").unwrap();
    let hgt = Regex::new(r"hgt:(.)+").unwrap();
    let hcl = Regex::new(r"hcl:(.)+").unwrap();
    let ecl = Regex::new(r"ecl:(.)+").unwrap();
    let pid = Regex::new(r"pid:(.)+").unwrap();
    let cid = Regex::new(r"cid:(.)+").unwrap();

    let byrm: bool = byr.is_match(passport);
    let iyrm: bool = iyr.is_match(passport);
    let eyrm: bool = eyr.is_match(passport);
    let hgtm: bool = hgt.is_match(passport);
    let hclm: bool = hcl.is_match(passport);
    let eclm: bool = ecl.is_match(passport);
    let pidm: bool = pid.is_match(passport);
    let cidm: bool = cid.is_match(passport);
    
    let valid: bool = byrm && iyrm && eyrm && hgtm && hclm && eclm && pidm;
    
    if !valid { 
        println!("invalid {}", passport); 
        println!("byr {} iyr {} eyr {} hgt {} hcl {} ecl {} pid {}\n", byrm, iyrm, eyrm, hgtm, hclm, eclm, pidm);
    }

    return valid;
}

pub fn run_ex_4_part_1() -> () {
    println!("Running exercise 4 part 1");

    let mut valid = 0;
    let mut curr_passport = String::from(" ");

    if let Ok(lines) = read_lines("./ex4.input.txt") {
        for line in lines {
            if line == "" {
                if is_valid(&curr_passport) {
                    valid += 1;
                }

                curr_passport = String::from(" ");
            } else {
                curr_passport = format!(" {} {} ", curr_passport, line);
            }
        }
    }

    println!("Result {}", valid);
}  