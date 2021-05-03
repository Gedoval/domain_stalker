pub mod menu;

use libloading::Library;
use plugins_core::{InvocationError, Plugin, PluginDeclaration};
use std::{alloc::System, collections::HashMap, env, ffi::OsStr, path::Path, rc::Rc};

use crate::menu::handlers::{exec, set, list};

#[global_allocator]
static ALLOCATOR: System = System;

pub fn run() -> Result<(), std::io::Error> {
    let ext_plugins = load_plugins();
    let mut prefix: Option<String> = None;
    let mut plugin_args: HashMap<String, String> = HashMap::new();

    loop {
        match menu::utils::read_command(prefix.clone())? {
            menu::parser::Command::Exit => break,
            menu::parser::Command::List => list::list_available_plugins(&ext_plugins),
            menu::parser::Command::Use(plugin) => {
                if ext_plugins.exists(plugin.as_str()) {
                    plugin_args.clear();
                    prefix.replace(plugin);
                } else {
                    eprintln!("Plugin {} not found", plugin);
                }
            },
            menu::parser::Command::Set(args) => {
                if prefix.is_none() {
                    eprintln!("[!] No plugin selected.");
                    continue;
                }
                set::handle_set(args, &mut plugin_args);
            },
            menu::parser::Command::Exec => exec::handle_exec(&ext_plugins, prefix.clone().unwrap().as_str(), &plugin_args),
            menu::parser::Command::Help(plugin) => {
                if plugin.is_empty() {
                    // TODO: Program help
                    println!("Program help");
                }
                else {
                    ext_plugins.show_help(&plugin).unwrap_or_else(
                    |err| eprintln!("{:?}", err)
                    );
                }
            },
            menu::parser::Command::Unknown(cmd) => eprintln!("{}", cmd),
            menu::parser::Command::Nothing => continue,
        };
    }

    return Ok(());
}

fn load_plugins() -> ExternalPlugins {
    let mut plugins = ExternalPlugins::new();

    let plugins_directory = match env::var("PLUGINS_DIRECTORY") {
        Ok(env_var) => env_var,
        Err(_) => env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .display()
            .to_string(),
    };
    let path = Path::new(&plugins_directory);

    println!("[*] Loading plugins from {}", &plugins_directory);

    unsafe {
        path.read_dir().unwrap().for_each(|dir_entry| {
            let file_path = dir_entry.unwrap().path().display().to_string();

            if file_path.ends_with("so") {
                plugins.load(file_path).expect("Function loading failed");
            }
        });
    }

    return plugins;
}

/// A map of all externally provided functions.
#[derive(Default)]
pub struct ExternalPlugins {
    plugins: HashMap<String, PluginProxy>,
    libraries: Vec<Rc<Library>>,
}

impl ExternalPlugins {
    fn new() -> ExternalPlugins {
        ExternalPlugins::default()
    }

    fn call(
        &self,
        plugin: &str,
        arguments: &std::collections::HashMap<String, String>,
    ) -> Result<(), InvocationError> {
        self.plugins
            .get(plugin)
            .ok_or_else(|| format!("\"{}\" not found", plugin))?
            .call(arguments.to_owned())
    }

    fn show_help(&self, plugin: &str) -> Result<(), InvocationError> {
        println!(
            "{}",
            self.plugins
                .get(plugin)
                .ok_or_else(|| format!("\"{}\" not found", plugin))?
                .help()
                .unwrap()
        );

        return Ok(());
    }

    fn exists(&self, plugin: &str) -> bool {
        return self.plugins.get(plugin).is_some();
    }

    /// Load a plugin library and add all contained functions to the internal
    /// function table.
    ///
    /// # Safety
    ///
    /// A plugin library **must** be implemented using the
    /// [`plugins_core::plugin_declaration!()`] macro. Trying manually implement
    /// a plugin without going through that macro will result in undefined
    /// behaviour.
    pub unsafe fn load<P: AsRef<OsStr>>(
        &mut self,
        library_path: P,
    ) -> Result<(), libloading::Error> {
        println!(
            "[*] Loading {} library",
            library_path.as_ref().to_string_lossy().to_string()
        );
        // load the library into memory
        let library = Rc::new(Library::new(library_path)?);

        // get a pointer to the plugin_declaration symbol.
        let decl = library
            .get::<*mut PluginDeclaration>(b"plugin_declaration\0")?
            .read();

        // version checks to prevent accidental ABI incompatibilities
        if decl.rustc_version != plugins_core::RUSTC_VERSION
            || decl.core_version != plugins_core::CORE_VERSION
        {
            panic!("Either loaded library or current rust version do not match");
        }

        let mut registrar = PluginRegistrar::new(Rc::clone(&library));

        (decl.register)(&mut registrar);

        // add all loaded plugins to the functions map
        self.plugins.extend(registrar.plugins);

        // and make sure ExternalFunctions keeps a reference to the library
        self.libraries.push(library);

        return Ok(());
    }
}

struct PluginRegistrar {
    plugins: HashMap<String, PluginProxy>,
    lib: Rc<Library>,
}

impl PluginRegistrar {
    fn new(lib: Rc<Library>) -> PluginRegistrar {
        PluginRegistrar {
            lib,
            plugins: HashMap::default(),
        }
    }
}

impl plugins_core::PluginRegistrar for PluginRegistrar {
    fn register_function(&mut self, name: &str, plugin: Box<dyn Plugin>) {
        let proxy = PluginProxy {
            plugin,
            _lib: Rc::clone(&self.lib),
        };
        self.plugins.insert(name.to_string(), proxy);
    }
}

/// A proxy object which wraps a [`Function`] and makes sure it can't outlive
/// the library it came from.
pub struct PluginProxy {
    plugin: Box<dyn Plugin>,
    _lib: Rc<Library>,
}

impl Plugin for PluginProxy {

    fn call(&self, args: std::collections::HashMap<String, String>) -> Result<(), InvocationError> {
        self.plugin.call(args)
    }

    fn help(&self) -> Option<&str> {
        self.plugin.help()
    }
}
