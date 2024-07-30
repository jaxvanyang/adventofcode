fn main() {
    let input = "day_4.input";
    let input = std::fs::read_to_string(input).unwrap();

    let mut lines = 0;

    for ch in input.chars() {
        if ch == ':' {
            lines += 1;
        }
    }

    let mut cards: Vec<usize> = vec![1; lines];

    for (i, line) in input.lines().enumerate() {
        let count = parse_line(line);

        if count == 0 {
            continue;
        }

        for j in (i + 1)..(i + 1 + count) {
            cards[j] += cards[i];
        }
    }

    let sum: usize = cards.iter().sum();

    println!("{sum}");
}

fn parse_line(line: &str) -> usize {
    let (winning, mine) = line.split_once('|').unwrap();
    let winning = &winning[(winning.find(':').unwrap() + 1)..];

    let winning: Vec<usize> = winning
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mine: Vec<usize> = mine
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut count: usize = 0;

    for i in winning {
        for j in &mine {
            if i == *j {
                count += 1;
            }
        }
    }

    count
}
