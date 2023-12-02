// use colorful::{Color, Colorful};

const DIGITS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", 

    "one",

    "two",
    "three",

    "four",
    "five",

    "six",
    "seven",

    "eight",
    "nine",
];

fn main() {
    let input = include_str!("input.txt");
    // let mut color = true;

    let answer: u32 = input
        .lines()
        .map(|l| {
            let mut idx = [-1; 100];
            for pat in DIGITS {
                let pat_starts = l.match_indices(pat).map(|(i, _)| i);
                for pat_start in pat_starts {
                    idx[pat_start] = parse(&l[pat_start..pat_start + pat.len()]) as i32;
                }
            }

            let arr: Vec<u32> = idx
                .into_iter()
                .filter(|n| *n != -1)
                .map(|i| i as u32)
                .collect();

            // indexes
            let a = *arr.first().unwrap();
            let b = *arr.last().unwrap();

            // let s = format!("{l}{} = {num}", " ".repeat(60 - l.len()), num = a * 10 + b);
            // println!(
            //     "{}",
            //     s.bg_color(match color {
            //         true => Color::Black,
            //         false => Color::Grey0,
            //     })
            // );

            // color = !color;

            a * 10 + b
        })
        .sum();

    println!("answer: {answer}");
}

fn parse(inp: &str) -> u32 {
    match inp {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,

        _ => panic!(),
    }
}
