use console::style;

use super::parser::Command;
use std::io::Write;

pub fn read_command(prefix: Option<String>) -> Result<Command, std::io::Error> {
    let plugin = match prefix {
        Some(p) => style(p),
        None => style(String::new()),
    };

    print!("{}{} ", plugin.bold().red(), style(">").bold());

    let _ = std::io::stdout().flush();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    Ok(Command::from(buffer.replace("\n", "")))
}
