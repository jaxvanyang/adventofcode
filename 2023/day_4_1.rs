fn main() {
    let input = "day_4.input";
    let input = std::fs::read_to_string(input).unwrap();

    let mut sum: u64 = 0;

    for line in input.lines() {
        sum += parse_line(line);
    }

    println!("{sum}");
}

fn parse_line(line: &str) -> u64 {
    let (winning, mine) = line.split_once('|').unwrap();
    let winning = &winning[(winning.find(':').unwrap() + 1)..];

    let winning: Vec<u32> = winning
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mine: Vec<u32> = mine
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut count: u64 = 0;

    for i in winning {
        for j in &mine {
            if i == *j {
                count += 1;
            }
        }
    }

    if count == 0 {
        0
    } else {
        1 << (count - 1)
    }
}
