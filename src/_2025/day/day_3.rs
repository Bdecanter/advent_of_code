pub fn part1(data: &str) -> i32 {
    let mut total :i32 = 0;

    for (_line_idx, line) in data.lines().enumerate() {
        let mut max_char: [i32; 2] = [0, 0];
        let mut max_char_idx: usize = 0;

        line.chars().enumerate().for_each(|(char_idx, char)| {
            let val = char.to_digit(10).unwrap() as i32;

            if max_char[0] < val && char_idx < line.len() - 1 {
                max_char[0] = val;
                max_char_idx = char_idx;
            }
        });

        let rest = &line[max_char_idx + 1..];
        rest.chars().enumerate().for_each(|(_char_idx, char)| {
            let val :i32 = char.to_digit(10).unwrap() as i32;

            if max_char[1] < val {
                max_char[1] = val;
            }
        });
        println!("{:?}", max_char);
        total += (max_char[0] * 10) + max_char[1];
    }

    total
}
