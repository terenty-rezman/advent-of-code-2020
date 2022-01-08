use std::error::Error;

type AnyError = Box<dyn Error>;

fn main() -> Result<(), AnyError> {
    let content = std::fs::read_to_string("input.txt")?;

    let lines: Vec<&str> = content.lines().collect();

    for i in 0..lines.len() {
        for j in i+1..lines.len() {
            let num1: u32 = lines[i].parse()?;
            let num2: u32 = lines[j].parse()?;
            if num1 + num2 == 2022 {
                println!("{}", num1 * num2);
                return Ok(());
            }
        }
    }

    Ok(())
}
