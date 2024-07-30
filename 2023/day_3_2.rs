use std::fs::read_to_string;
use std::str::from_utf8;

fn main() {
    let file = "day_3.input";
    let input = read_to_string(file).unwrap();
    let input = to_byte_matrix(&input);

    let mut sum = 0;

    for (i, line) in input.iter().enumerate() {
        sum += parse_line(i, line, &input);
    }

    println!("{sum}");
}

fn to_byte_matrix(input: &str) -> Vec<&[u8]> {
    input.lines().map(|line| line.as_bytes()).collect()
}

fn parse_line(i: usize, line: &[u8], input: &Vec<&[u8]>) -> u32 {
    let mut sum = 0;

    for (j, byte) in line.iter().enumerate() {
        if *byte == b'*' {
            sum += calc_ratio(i, j, input);
        }
    }

    sum
}

fn calc_ratio(i: usize, j: usize, input: &Vec<&[u8]>) -> u32 {
    let height = input.len();
    let width = input[0].len();
    let mut numbers = Vec::new();

    if i > 0 {
        if input[i - 1][j].is_ascii_digit() {
            if let Some(num) = search(i - 1, j, input) {
                numbers.push(num);
            }
        } else {
            if j > 0 {
                if let Some(num) = search(i - 1, j - 1, input) {
                    numbers.push(num);
                }
            }
            if j + 1 < width {
                if let Some(num) = search(i - 1, j + 1, input) {
                    numbers.push(num);
                }
            }
        }
    }

    if j > 0 {
        if let Some(num) = search(i, j - 1, input) {
            numbers.push(num);
        }
    }
    if j + 1 < width {
        if let Some(num) = search(i, j + 1, input) {
            numbers.push(num);
        }
    }

    if i + 1 < height {
        if input[i + 1][j].is_ascii_digit() {
            if let Some(num) = search(i + 1, j, input) {
                numbers.push(num);
            }
        } else {
            if j > 0 {
                if let Some(num) = search(i + 1, j - 1, input) {
                    numbers.push(num);
                }
            }
            if j + 1 < width {
                if let Some(num) = search(i + 1, j + 1, input) {
                    numbers.push(num);
                }
            }
        }
    }

    if numbers.len() == 2 {
        numbers[0] * numbers[1]
    } else {
        0
    }
}

fn search(i: usize, j: usize, input: &Vec<&[u8]>) -> Option<u32> {
    if !input[i][j].is_ascii_digit() {
        return None;
    }

    let len = input[0].len();
    let mut l = j;
    let mut r = j + 1;

    while l > 0 && input[i][l - 1].is_ascii_digit() {
        l -= 1;
    }

    while r < len && input[i][r].is_ascii_digit() {
        r += 1;
    }

    Some(from_utf8(&input[i][l..r]).unwrap().parse().unwrap())
}
