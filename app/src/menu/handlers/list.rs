use crate::ExternalPlugins;


pub fn list_available_plugins(ext_plugins: &ExternalPlugins) {
    &ext_plugins.plugins
        .iter()
        .for_each(
            |(key, value)| println!("{} {:?}", key, value.plugin.help().unwrap())
        );
}