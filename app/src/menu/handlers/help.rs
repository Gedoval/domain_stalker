use plugins_core::{InvocationError, Plugin};

use cli_table::{Cell, Style, Table, format::Justify, print_stdout};

use crate::ExternalPlugins;

pub fn show_plugin_help(
    plugin_name: &str,
    ext_plugins: &ExternalPlugins,
) -> Result<(), InvocationError> {
    let help = ext_plugins.plugins.borrow();
    let help = help
        .get(plugin_name)
        .ok_or_else(|| format!("\"{}\" not found", plugin_name))?
        .help();

    let mut body = vec![
        vec!["Plugin name".cell(), plugin_name.cell()],
        vec!["Description".cell(), help.description.cell()],
    ];
    body.extend(
        help.args
            .iter()
            .map(|arg| vec![arg.name.cell(), arg.desc.cell()])
            .collect::<Vec<_>>(),
    );

    let table = body.table()
        .title(
            vec![
                "Descriptor".cell().justify(Justify::Center).bold(true),
                "Descriptor value".cell().justify(Justify::Center).bold(true),
            ],
        );

    print_stdout(table).unwrap();

    Ok(())
}
