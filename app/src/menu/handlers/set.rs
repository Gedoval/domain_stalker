pub fn handle_set(args: Vec<String>, plugin_args: &mut std::collections::HashMap<String, String>) {
    let var = match args.get(0) {
        Some(v) => v.to_string(),
        None => {
            eprintln!("[!] Variable missing");
            return;
        }
    };

    let value = match args.get(1) {
        Some(v) => v.to_string(),
        None => {
            eprintln!("[!] Variable value missing");
            return;
        }
    };

    plugin_args.insert(var, value);
}