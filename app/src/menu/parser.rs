pub enum Command {
    Exit,
    List,
    Set(Vec<String>),
    Use(String),
    Exec,
    Unknown(String),
    Help(String),
    Nothing
}


impl From<String> for Command {
    fn from(cmd: String) -> Self {
        let mut split_whitespace = cmd.split_whitespace();

        return match split_whitespace.next() {
            Some(cmd) => match cmd {
                "exit" => Self::Exit,
                "list" => Self::List,
                "use" => Self::Use(split_whitespace.map(String::from).collect::<Vec<String>>().join(" ")),
                "set" => Self::Set(split_whitespace.map(String::from).collect::<Vec<String>>()),
                "exec" => Self::Exec,
                "help" => Self::Help(split_whitespace.map(String::from).collect::<Vec<String>>().join(" ")),
                _ => Self::Unknown(format!("Unknown command: {}", &cmd))
            },
            None => Self::Nothing
        };
    }
}
