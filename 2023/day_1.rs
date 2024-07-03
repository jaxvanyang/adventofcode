use std::fs::read_to_string;

fn main() {
    let filename = "day_1.input";
    let mut sum: u32 = 0;

    for line in read_to_string(filename).unwrap().lines() {
        sum += parse(line);
    }

    println!("{sum}");
}

fn parse(line: &str) -> u32 {
    let mut l: u32 = 0;
    let mut r: u32 = 0;

    for ch in line.chars() {
        match ch.to_digit(10) {
            Some(digit) => {
                if l == 0 {
                    l = digit;
                }
                r = digit;
            },
            None => {},
        }
    }

    l * 10 + r
}
