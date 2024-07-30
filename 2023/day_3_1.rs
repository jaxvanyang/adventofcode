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
    let len = line.len();
    let mut l: usize = 0;
    let mut r: usize;
    let mut sum = 0;

    while l < len {
        if line[l] == b'.' || !line[l].is_ascii_digit() {
            l += 1;
            continue;
        }

        r = l + 1;

        while r < len {
            if !line[r].is_ascii_digit() {
                break;
            }
            r += 1;
        }

        let num: u32 = from_utf8(&line[l..r]).unwrap().parse().unwrap();

        if check(i, len, l, r, input) {
            sum += num;
        }

        l = r + 1;
    }

    sum
}

fn check(i: usize, line_len: usize, l: usize, r: usize, input: &Vec<&[u8]>) -> bool {
    let matrix_len = input.len();
    let t = if i > 0 { i - 1 } else { i };
    let b = if i + 1 == matrix_len { i + 1 } else { i + 2 };
    let l = if l > 0 { l - 1 } else { l };
    let r = if r < line_len { r + 1 } else { r };

    for i in t..b {
        for j in l..r {
            if input[i][j] != b'.' && !input[i][j].is_ascii_digit() {
                return true;
            }
        }
    }

    false
}
