pub fn part1(data: &str) -> i32 {
    println!("Starting decode part one");

    // we start at 50
    let mut state: i32 = 50;
    let mut nb_of_zero: i32 = 0;

    data.lines().enumerate().for_each(|(y, line)| {
        let l = line[1..].parse::<i32>().unwrap();

        if line.chars().nth(0).unwrap() == 'L' {
            state = state - l;

        } else {
            state = state + l;
        }

        while state < 0 || state > 99 {
            if state > 99 {
                state = state - 100;
            }

            if state < 0 {
                state = state + 100;
            }

            if state < 99 && state > 0 {
                continue
            }
        }

        if state == 0 {
            nb_of_zero = nb_of_zero + 1;
        }
    });
    nb_of_zero
}

pub fn part2(data: &str) -> i32 {
    println!("Starting decode part two");

    // we start at 50
    let mut state: i32 = 50;
    let mut nb_of_zero: i32 = 0;
    let mut state_was_zero: bool = false;

    data.lines().enumerate().for_each(|(y, line)| {
        let l = line[1..].parse::<i32>().unwrap();
        println!("Line {}: State {} - {}", line, state, l);

        if state == 0 {
            state_was_zero = true;
        } else {
            state_was_zero = false;
        }

        if line.chars().nth(0).unwrap() == 'L' {
            state = state - l;

        } else {
            state = state + l;
        }

        while state < 0 || state > 99 {

            if state == 100 {
                state = 0;
            }

            if state > 99 {
                state = state - 100;
                println!("ok un tour");

                if !state_was_zero {
                    nb_of_zero = nb_of_zero + 1;
                }

            }

            if state < 0 {
                state = state + 100;
                println!("ok un tour");
                if !state_was_zero {
                    nb_of_zero = nb_of_zero + 1;
                }
            }

            state_was_zero = false;
            // if state < 99 && state > 0 {
            //     continue
            // }
        }

        if state == 0 {
            nb_of_zero = nb_of_zero + 1;
            println!("ZERO");
        }
    });
    nb_of_zero

}