use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let grid: Vec<&str> = input.trim().split('\n').collect();

    let mut cs = HashSet::new();

    for (y, line) in grid.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '.' || ch.is_ascii_digit() {
                continue;
            }

            let y = y as isize;
            let x = x as isize;

            for dy in [y - 1, y, y + 1] {
                for dx in [x - 1, x, x + 1] {
                    if dy < 0
                        || dy as usize >= grid.len()
                        || dx < 0
                        || dx as usize >= grid[dy as usize].len()
                        || !grid[dy as usize]
                            .chars()
                            .nth(dx as usize)
                            .unwrap()
                            .is_ascii_digit()
                    {
                        continue;
                    }

                    let mut dx = dx as usize;

                    while dx > 0
                        && grid[dy as usize]
                            .chars()
                            .nth(dx - 1)
                            .unwrap()
                            .is_ascii_digit()
                    {
                        dx -= 1;
                    }

                    cs.insert((dy, dx));
                }
            }
        }
    }

    let mut ns: Vec<usize> = vec![];
    for &(r, c) in cs.iter() {
        let mut s = String::new();

        let mut c = c;
        while c < grid[r as usize].len()
            && grid[r as usize].chars().nth(c).unwrap().is_ascii_digit()
        {
            s.push(grid[r as usize].chars().nth(c).unwrap());
            c += 1;
        }

        ns.push(s.parse().unwrap());
    }

    println!("answer: {answer}", answer = ns.into_iter().sum::<usize>());
}
