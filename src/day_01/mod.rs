use std::collections::HashMap;

pub fn part_one(input: &str) -> i32 {
    let mut ans: i32 = 0;
    let mut switch = true;
    let mut l_list: Vec<i32> = vec![];
    let mut r_list: Vec<i32> = vec![];
    for line in input.lines() {
        for part in line.split_whitespace() {
            if let Ok(n) = part.parse::<i32>() {
                match switch {
                    true => {
                        l_list.push(n);
                        switch = false;
                    }
                    false => {
                        r_list.push(n);
                        switch = true;
                    }
                };
            }
        }
    }
    l_list.sort();
    r_list.sort();
    for (l_val, r_val) in l_list.iter().zip(r_list.iter()) {
        ans += (l_val - r_val).abs();
    }
    ans
}

pub fn part_two(input: &str) -> i32 {
    let mut ans: i32 = 0;
    let mut switch = true;
    let mut l_list: Vec<i32> = vec![];
    let mut r_list: Vec<i32> = vec![];
    for line in input.lines() {
        for part in line.split_whitespace() {
            if let Ok(n) = part.parse::<i32>() {
                match switch {
                    true => {
                        l_list.push(n);
                        switch = false;
                    }
                    false => {
                        r_list.push(n);
                        switch = true;
                    }
                };
            }
        }
    }
    l_list.sort();
    r_list.sort();
    // this is where i left off
    // i was trying to create a hashmap out of the right list
    // where the keys are the values of r_list
    // and the values of those keys are the number of occurences "key" has in r_list
    2
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_one_sample() {
        let input: String = match fs::read_to_string("./src/day_01/sample.txt") {
            Ok(s) => s,
            Err(_) => panic!("Could not locate sample.txt."),
        };
        let output = 11;
        assert_eq!(part_one(&input), output);
    }

    #[test]
    fn test_one() {
        let input: String = match fs::read_to_string("./src/day_01/part_one.txt") {
            Ok(s) => s,
            Err(_) => panic!("Could not locate part_one.txt."),
        };
        let output = 1580061;
        assert_eq!(part_one(&input), output);
    }

    #[test]
    fn test_two_sample() {
        let input: String = match fs::read_to_string("./src/day_01/sample.txt") {
            Ok(s) => s,
            Err(_) => panic!("Could not locate sample.txt."),
        };
        let output = 31;
        assert_eq!(part_two(&input), output);
    }
}
