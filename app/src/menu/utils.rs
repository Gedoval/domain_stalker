use super::parser::Command;
use std::io::Write;


pub fn read_command(prefix: Option<String>) -> Result<Command, std::io::Error> {
    print!("{}> ", match prefix {
        Some(p) => p,
        None => String::new()
    });

    let _ = std::io::stdout().flush();
    
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    return Ok(Command::from(buffer.replace("\n", "")));
}