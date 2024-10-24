use std::{error::Error, io::stdin};

pub struct Terminal {}

impl Terminal {
    // One and only input handler
    pub fn get_input() -> Result<String, &'static str> {
        loop {
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Error reading input");

            let trimmed_input = input.trim();

            if trimmed_input == "!q" {
                return Err("input mode quit");
            }
            if trimmed_input.is_empty() {
                println!("failed to read input, try again or type !q to quit input mode");
                continue;
            } else {
                return Ok(trimmed_input.to_string());
            }
        }
    }
    // Should always be called after get_input
    pub fn handle_command(cmd: &str) -> Result<(), Box<dyn Error>> {
        match cmd {
            "w" => println!("W!"),
            "e" => todo!(),
            "r" => todo!(),
            "q" => todo!(),
            _ => todo!(),
        }
        Ok(())
    }

    pub fn clear() {
        print!("\x1B[2J\x1B[1;1H");
    }
}
