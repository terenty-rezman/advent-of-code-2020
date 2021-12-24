use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    let fields: HashSet<_> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
        .into_iter()
        .collect();

    println!(
        "{}",
        input
            .split("\n\r")
            .filter(|l| {
                let mut parsed_fields: HashSet<_> = l
                    .split_whitespace()
                    .map(|field_value| field_value.split(':').next().unwrap())
                    .collect();

                parsed_fields.insert("cid");
                parsed_fields == fields
            })
            .count()
    );
}
