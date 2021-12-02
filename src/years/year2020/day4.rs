use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;
use std::error;

fn solve_part_one(passports: Vec<Passport>) -> usize {
    passports.iter().filter(|&x| x.is_valid()).count()
}

fn solve_part_two(passports: Vec<Passport>) -> usize {
    passports.iter().filter(|&x| x.has_valid_values().unwrap()).count()
}

#[derive(Debug)]
struct Passport {
    dictionary: HashMap<String, String>
}

impl Passport {
    fn parse_passport(content: &str) -> Self {
        let res = content.split_ascii_whitespace().map(|kvp|
            {
                let split = kvp.split(":").collect::<Vec<_>>();
                (split[0].to_string(), split[1].to_string())
            }
        ).collect::<HashMap<String, String>>();
        // println!("dict was {:?}", res);

        Self {
            dictionary: res
        }
    }

    fn is_valid(&self) -> bool {
        let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        // let optional = ["cid"];
        required.iter().all(|&key| self.dictionary.contains_key(key))
    }

    fn has_valid_values(&self) -> Result<bool, Box<dyn error::Error>> {
        if !self.is_valid() {
            return Ok(false);
        }

        let bie: Vec<_> = ["byr", "iyr", "eyr"].iter()
            .map(|&k| usize::from_str(&self.dictionary[k]))
            .collect::<Result<Vec<_>, _>>()?
            ;

        let byr = 1920 <= bie[0] && 2002 >= bie[0];
        let iyr = 2010 <= bie[1] && 2020 >= bie[1];
        let eyr = 2020 <= bie[2] && 2030 >= bie[2];

        let re_hcl = Regex::new(r"^#[0-9a-f]{6}$")?;
        let hcl = re_hcl.is_match(&self.dictionary["hcl"]);

        let valid_ecl = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        let ecl = valid_ecl.contains(&self.dictionary["ecl"].as_str());

        let re_pid = Regex::new(r"^[0-9]{9}$")?;
        let pid = re_pid.is_match(&self.dictionary["pid"]);

        let re_hgt = Regex::new(r"^(\d{2,3})(in|cm)$")?;
        if !re_hgt.is_match(&self.dictionary["hgt"]) {
            return Ok(false);
        }
        let caps = re_hgt.captures(&self.dictionary["hgt"])
            .ok_or("hgt invalid")?;
        let metric_hgt = caps.get(2).map_or("", |m| m.as_str());
        let val_hgt = usize::from_str(caps.get(1).map_or("", |m| m.as_str()))?;
        let hgt;
        if metric_hgt == "in" {
            hgt = 59 <= val_hgt && 76 >= val_hgt;
        } else {
            hgt = 150 <= val_hgt && 193 >= val_hgt;
        }

        Ok([byr, iyr, eyr, hgt, hcl, ecl, pid].iter().all(|&x| x))
    }
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;
    use std::io::Error;
    use super::*;
    use std::error;


    #[test]
    fn test_part_one() -> Result<(), Error> {
        let input = read_to_string("../../../inputs/year2020/day4.txt")?;
        let lines = input.lines();
        let mut cur_lines = String::new();
        let mut passports: Vec<Passport> = vec![];
        for line in lines {
            if line.trim().is_empty() {
                // println!("now parsing {}", cur_lines);
                passports.push(Passport::parse_passport(&*cur_lines));
                cur_lines.clear();
            } else {
                cur_lines = format!("{} {}", cur_lines, line);
            }
        }
        let solution = solve_part_one(passports);
        println!("[day4] solution part 1: {}", solution);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), Box<dyn error::Error>> {
        let input = read_to_string("../../../inputs/year2020/day4.txt")?;
        let lines = input.lines();
        let mut cur_lines = String::new();
        let mut passports: Vec<Passport> = vec![];
        for line in lines {
            if line.trim().is_empty() {
                // println!("now parsing {}", cur_lines);
                passports.push(Passport::parse_passport(&*cur_lines));
                cur_lines.clear();
            } else {
                cur_lines = format!("{} {}", cur_lines, line);
            }
        }
        let solution = solve_part_two(passports);
        println!("[day4] solution part 2: {}", solution);
        Ok(())
    }
}