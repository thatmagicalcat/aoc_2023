fn main() {
    let input = include_str!("input.txt");
    let grid: Vec<&str> = input.trim().split('\n').collect();
    let mut total = 0;

    for (y, line) in grid.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '*' {
                continue;
            }
            let mut cs = std::collections::HashSet::new();

            for &yd in &[y - 1, y, y + 1] {
                for &xd in &[x - 1, x, x + 1] {
                    if 
                        yd >= grid.len()
                        || xd >= grid[yd].len()
                        || !grid[yd].chars().nth(xd).map_or(false, |c| c.is_digit(10))
                    {
                        continue;
                    }

                    let mut xd = xd;

                    while xd > 0 && grid[yd].chars().nth(xd - 1).map_or(false, |c| c.is_digit(10)) {
                        xd -= 1;
                    }

                    cs.insert((yd, xd));
                }
            }

            if cs.len() != 2 {
                continue;
            }

            let mut ns = Vec::new();

            for (r, mut c) in cs {
                let mut s = String::new();

                while c < grid[r].len() && grid[r].chars().nth(c).map_or(false, |c| c.is_digit(10)) {
                    s.push(grid[r].chars().nth(c).unwrap());
                    c += 1;
                }

                ns.push(s.parse::<i32>().unwrap());
            }

            total += ns[0] * ns[1];
        }
    }

    println!("answer: {}", total);
}