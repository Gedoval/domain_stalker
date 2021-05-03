use crate::ExternalPlugins;

pub fn handle_exec(ext_plugins: &ExternalPlugins, plugin: &str, plugin_args: &std::collections::HashMap<String, String>) {
    if let Err(err) = ext_plugins.call(plugin, &plugin_args) {
        eprintln!("[!] Error: {:?}", err)
    };
}