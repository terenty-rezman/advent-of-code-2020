use regex::Regex;

struct Password<'password> {
    min: usize,
    max: usize,
    letter: char,
    password: &'password str,
}

fn main() {
    let input = include_str!("input.txt");
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();

    let count = re
        .captures_iter(input)
        .filter_map(|cap| {
            let groups = (cap.get(1), cap.get(2), cap.get(3), cap.get(4));

            match groups {
                (Some(min), Some(max), Some(letter), Some(password)) => Some(Password {
                    min: min.as_str().parse().unwrap(),
                    max: max.as_str().parse().unwrap(),
                    letter: letter.as_str().chars().collect::<Vec<char>>()[0],
                    password: password.as_str(),
                }),
                _ => None,
            }
        })
        .fold(0, right_policy);

    dbg!(count);
}

fn wrong_policy(valid: u32, p: Password) -> u32 {
    let count = p.password.matches(p.letter).count();
    if p.min <= count && count <= p.max {
        valid + 1
    } else {
        valid
    }
}

fn right_policy(valid: u32, p: Password) -> u32 {
    let mut count = 0;
    for i in [p.min, p.max] {
        if p.password.chars().nth(i - 1).unwrap() == p.letter {
            count += 1;
        }
    }

    if count == 1 {
        valid + 1
    } else {
        valid
    }
}