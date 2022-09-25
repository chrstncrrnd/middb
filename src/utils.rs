use std::io::{self, Write};

pub fn get_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    let _ = std::io::stdout().flush();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Error reading from stdin");
    Ok(buffer.trim().to_owned())
}
