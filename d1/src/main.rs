use cli_clipboard;

fn read_input() -> String {
    let input = cli_clipboard::get_contents().unwrap();

    //println!("{}", input);

    input
}

fn write_output(output: String) {
    println!("{}", output);

    let _result = cli_clipboard::set_contents(output);
}

fn main() {
    let _input = read_input();

    let _output = d2b(_input);

    write_output(_output);
}

fn d2b(input: String) -> String {
    let mut sum_powers: i32 = 0;

    for line in input.lines() {
        let (game_name, sets) = d2_split_game_details(line);

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for set in sets {
            let quantities = d2_colors_and_numbers(&set);

            for (color, quantity) in quantities {
                if color == "red" {
                    if quantity > red {
                        red = quantity;
                    }
                } else if color == "green" {
                    if quantity > green {
                        green = quantity;
                    }
                } else if color == "blue" {
                    if quantity > blue {
                        blue = quantity;
                    }
                }
            }
        }

        let game_id = game_name.split_whitespace().nth(1).unwrap_or("0").parse::<i32>().unwrap_or(0);

        sum_powers += red * green * blue;
    }

    return sum_powers.to_string();
}

fn d2a(input: String) -> String {
    let mut sum_ids: i32 = 0;

    for line in input.lines() {
        let (game_name, sets) = d2_split_game_details(line);

        let mut valid = true;

        for set in sets {
            let mut red = 12;
            let mut green = 13;
            let mut blue = 14;
            
            let quantities = d2_colors_and_numbers(&set);

            for (color, quantity) in quantities {
                if color == "red" {
                    red -= quantity
                } else if color == "green" {
                    green -= quantity;
                } else if color == "blue" {
                    blue -= quantity;
                }
            }

            if red < 0 || green < 0 || blue < 0 {
                valid = false;
                break;
            }
        }

        let game_id = game_name.split_whitespace().nth(1).unwrap_or("0").parse::<i32>().unwrap_or(0);

        if valid {
            // valid game, let's add the game id into the sum:

            println!("valid: {}", game_id);

            sum_ids += game_id;
        } else {
            println!("invalid: {}", game_id);
        }
    }

    return sum_ids.to_string();
}

fn d2_split_game_details(line: &str) -> (String, Vec<String>) {
    let parts: Vec<&str> = line.splitn(2, ':').collect();
    let game_name = parts[0].trim().to_string();
    let groups = parts[1]
        .split(';')
        .map(|s| s.trim().to_string())
        .collect();

    (game_name, groups)
}

fn d2_colors_and_numbers(input: &str) -> Vec<(String, i32)> {
    input.split(',')
        .filter_map(|pair| {
            let parts: Vec<&str> = pair.trim().split_whitespace().collect();
            if parts.len() == 2 {
                let quantity = parts[0].parse::<i32>().ok()?;
                let color = parts[1].to_string();
                Some((color, quantity))
            } else {
                None
            }
        })
        .collect()
}

fn d1a(input: String) -> String {
    let mut sum = 0;

    for line in input.lines() {
        let first_digit = line.chars().find(|c| c.is_digit(10)).unwrap().to_digit(10).unwrap();
        let last_digit = line.chars().rev().find(|c| c.is_digit(10)).unwrap().to_digit(10).unwrap();

        sum += first_digit * 10 + last_digit;
    }

    return sum.to_string();
}

fn d1b(input: String) -> String {
    let mut output: String = String::new();

    let digit_mappings = [
        ("zero", "0"), ("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), 
        ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")
    ];

    for line in input.lines() {
        let mut out_line = String::new();
        let chars: Vec<char> = line.chars().collect();

        let mut i = 0;

        while i < chars.len() {
            if chars[i].is_digit(10) {
                out_line.push(chars[i]);
            } else {
                for &(word, digit) in &digit_mappings {
                    if chars[i..].iter().collect::<String>().starts_with(word) {
                        out_line.push_str(digit);
                        break;
                    }
                }
            }

            i += 1;
        }

        output.push_str(&out_line);
        output.push('\n');
    }

    println!("{}", output);

    return d1a(output);
}