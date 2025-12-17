use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt")
        .expect("Should have been able to read the file");

    let mut local_num = String::from("");
    let mut nums: Vec<i64> = vec![];

    for char in input.chars() {
        match char {
            ',' | '-' => {
                let num: i64 = local_num.trim().parse().unwrap();
                nums.push(num);
                local_num = String::from("");
            }
            _ => {
                local_num.push(char);
            }
        }
    }

    let num: i64 = local_num.trim().parse().unwrap();
    nums.push(num);

    part1(&nums);
    part2(&nums);
}

fn part1(nums: &[i64]) {
    let mut result = 0;

    for pair in nums.chunks(2) {
        let start = pair[0];
        let end =  pair[1] + 1;

        for i in start..end {
            let as_string: String = i.to_string();
            let half_len = as_string.chars().count() / 2;

            let part1: String = as_string.chars().take(half_len).collect();
            let part2: String = as_string.chars().skip(half_len).collect();

            if part1 == part2 {
                //println!("{}", i);
                result += i;
            }
        }
    }

    println!("Part1: {}", result);
}

fn part2(nums: &[i64]) {
    let mut result = 0;

    for pair in nums.chunks(2) {
        let start = pair[0];
        let end =  pair[1] + 1;

        for i in start..end {
            let as_string: String = i.to_string();
            let repeating: bool = is_repeating(&as_string);

            if repeating {
                result += i;
            }
        }
    }

    println!("Part2: {}", result);
}

fn is_repeating(s: &str) -> bool {
    let string_length = s.len();

    for substr_length in 1..=(string_length/2) {
        if string_length % substr_length != 0 {
            continue;
        }

        let substr = &s[..substr_length];

        if substr.repeat(string_length / substr_length) == s {
            return true;
        }
    }

    return false;
}
