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

    let _output = d1b(_input);

    write_output(_output);
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