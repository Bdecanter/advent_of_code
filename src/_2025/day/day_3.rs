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

pub fn part2(data: &str) -> i32 {
    let mut total :i32 = 0;

    for (_line_idx, line) in data.lines().enumerate() {
        let max_digit :i32 = 12;
        let mut max_value: i32 = 0;
        let mut max_value_tab: Vec<i32> = vec!();
        let mut max_char_idx: usize = 0;

        // we get the max value but keeping at least 12 digit
        line.chars().enumerate().for_each(|(char_idx, char)| {
            let val = char.to_digit(10).unwrap() as i32;

            if max_value < val && char_idx < line.len() - max_digit as usize {
                max_value = val;
                max_char_idx = char_idx;
            }
        });

        let rest = &line[max_char_idx + 1..];

        println!("idx:{} -> {}", max_char_idx, max_value);
        println!("Reste: {}", rest);

        let mut next :i32 = 0;
        let mut next_idx :usize = 0;
        let mut digit_left :i32 = max_digit - 1;

        for i in (0..max_digit - 1).rev() {
            println!("{} -> {}", i, digit_left);

            rest.chars().enumerate().for_each(|(char_idx, char)| {
                let val :i32 = char.to_digit(10).unwrap() as i32;

                if val > next && char_idx < line.len() - digit_left as usize {
                    next = val;
                    next_idx = char_idx;
                }
            })





/*
            if digit_left <= i {
                println!("++++ digit left {}", digit_left);
                break;
            }
            rest.chars().enumerate().for_each(|(char_idx, char)| {
                let val :i32 = char.to_digit(10).unwrap() as i32;

                if val > next && char_idx < line.len() - digit_left as usize {
                    next = val;
                    next_idx = char_idx;
                }
            })
*/
        }

    }
1
}
