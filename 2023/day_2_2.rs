use std::fs::read_to_string;

fn main() {
    let filename = "day_2.input";
    let mut sum: u32 = 0;

    for line in read_to_string(filename).unwrap().lines() {
        sum += parse(line);
    }

    println!("{sum}");
}

fn parse_cube(cube: &str) -> (&str, u32) {
    let mut iter = cube.split_whitespace();
    let count = iter.next().unwrap().parse::<u32>().unwrap();
    let cube = iter.next().unwrap();

    (cube, count)
}

fn parse_cubes(cubes: &str) -> (u32, u32, u32) {
    let (mut reds, mut greens, mut blues) = (0, 0, 0);

    for cube in cubes.split(',') {
        let (cube, count) = parse_cube(cube);

        if cube == "red" {
            reds = reds.max(count);
        } else if cube == "green" {
            greens = greens.max(count);
        } else {
            blues = blues.max(count);
        }
    }

    (reds, greens, blues)
}

fn parse(line: &str) -> u32 {
    let collection = line.split_once(':').unwrap().1;
    let (mut reds, mut greens, mut blues) = (0, 0, 0);

    for cubes in collection.split(';') {
        let (red, green, blue) = parse_cubes(cubes);
        reds = reds.max(red);
        greens = greens.max(green);
        blues = blues.max(blue);
    }

    reds * greens * blues
}
