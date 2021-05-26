use crate::ExternalPlugins;
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use plugins_core::Plugin;

const HEADERS: [&str; 2] = ["Plugin name", "Description"];

pub fn list_available_plugins(ext_plugins: &ExternalPlugins) {
    let plugins = ext_plugins.plugins.borrow();
    let table = plugins.iter()
        .map(|(key, plugin)| vec![key.cell(), plugin.description().cell()])
        .table()
        .title(HEADERS.iter().map(|header| header.cell().bold(true).justify(Justify::Center)))
        .bold(true);
    
    print_stdout(table).unwrap();
}
