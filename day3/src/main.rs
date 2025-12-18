use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("src/input.txt").unwrap();
    let lines = contents.lines();

    let mut result: u32 = 0;

    for line in lines {
        let line_chars: Vec<char> = line.chars().collect();

        let first: u32 = line_chars[0].to_digit(10).unwrap();
        let second: u32 = line_chars[1].to_digit(10).unwrap();

        let mut nums: [u32; 2] = [first, second];

        let chars: Vec<char> = line.chars().collect::<Vec<char>>();
        let count: usize = chars.len();

        let mut i = 1;

        while i < count {
            let current: u32 = chars[i].to_digit(10).unwrap() as u32;
            if i < count - 1 {
                if current > nums[0] {
                    let next: u32 = chars[i+1].to_digit(10).unwrap() as u32;
                    nums[0] = current;
                    nums[1] = next;
                    i += 1;
                    continue;
                }
            }

            if current > nums[1] {
                nums[1] = current;
            }

            i += 1;
        }

        let nums_string = nums 
            .iter()
            .map(|&n| n.to_string())
            .collect::<Vec<String>>()
            .join("");


        //println!("{}", nums_string);
        //println!("--------------------");

        result += nums_string.parse::<u32>().unwrap();
    }

    println!("Part 1: {}", result);
    
}
