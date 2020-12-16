use regex::Regex;
use crate::common::read_lines;

fn is_valid_simple(passport: &str) -> bool {
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

    return valid;
}

fn is_valid(passport: &str) -> bool {
    let byr = Regex::new(r"byr:(\d\d\d\d)").unwrap();
    let iyr = Regex::new(r"iyr:(\d\d\d\d)").unwrap();
    let eyr = Regex::new(r"eyr:(\d\d\d\d)").unwrap();
    let hgt = Regex::new(r"hgt:(\d+)(cm|in)").unwrap();
    let hcl = Regex::new(r"hcl:#([0-9a-f]{6})").unwrap();
    let ecl = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
    let pid = Regex::new(r"pid:(\d{9})").unwrap();
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
        return false;
    }

    // byr validation
    let byr_cap = byr.captures(passport).unwrap();
    let byr_value: i32 = byr_cap[1].parse().unwrap();

    if byr_value < 1920 || byr_value > 2002 {
        return false;
    }

    // iyr validation
    let iyr_cap = iyr.captures(passport).unwrap();
    let iyr_value: i32 = iyr_cap[1].parse().unwrap();

    if iyr_value < 2010 || iyr_value > 2020 {
        return false;
    }

    // eyr validation
    let eyr_cap = eyr.captures(passport).unwrap();
    let eyr_value: i32 = eyr_cap[1].parse().unwrap();

    if eyr_value < 2020 || eyr_value > 2030 {
        return false;
    }

    
    // hgt validation
    let hgt_cap = hgt.captures(passport).unwrap();
    let hgt_value: i32 = hgt_cap[1].parse().unwrap();
    let hgt_unit: &str = &hgt_cap[2];
    
    if (hgt_unit == "cm" && (hgt_value < 150 || hgt_value > 193)) 
    || (hgt_unit == "in" && (hgt_value < 59 || hgt_value > 76)) {
        return false
    }

    return true;
}

pub fn run_ex_4_part_1() -> () {
    println!("Running exercise 4 part 1");

    let mut valid = 0;
    let mut curr_passport = String::from(" ");

    if let Ok(lines) = read_lines("./ex4.input.txt") {
        for line in lines {
            if line == "" {
                if is_valid_simple(&curr_passport) {
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

pub fn run_ex_4_part_2() -> () {
    println!("Running exercise 4 part 2");

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