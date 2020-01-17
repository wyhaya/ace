/// ## Initialize command line parsing

/// ### Example
/// ```rust
/// use ace::App;
///
/// let app = App::new()
///     .config("app", env!("CARGO_PKG_VERSION"))
///     .desc("This is a description")
///     .cmd("start", "Start now")
///     .cmd("help", "Display help information")
///     .cmd("version", "Display version information")
///     .opt("--config", "Use configuration file")
///     .opt("--duration", vec!["Set duration of test", "example (1ms, 1s, 1m, 1h, 1d)"])
///     .opt("--timeout", "Set timeout");
///
/// if let Some(cmd) = app.command() {
///     match cmd.as_str() {
///         "start" => {
///             dbg!(app.value("--config"));
///         }
///         "help" => {
///             app.print_help();
///         }
///         "version" => {
///             app.print_version();
///         }
///         _ => {
///             app.print_error_try("help");
///         }
///     }
/// } else {
///     dbg!(app.args());
/// }
/// ```

#[derive(Debug, Clone, Default)]
pub struct App<'a> {
    args: Vec<String>,
    name: Option<&'a str>,
    version: Option<&'a str>,
    desc: Option<&'a str>,
    command: Vec<(&'a str, Vec<String>)>,
    option: Vec<(&'a str, Vec<String>)>,
}

impl<'a> App<'a> {
    /// Create
    pub fn new() -> App<'a> {
        App {
            args: std::env::args().collect::<Vec<String>>(),
            name: None,
            version: None,
            desc: None,
            command: vec![],
            option: vec![],
        }
    }

    /// Set name
    pub fn name(mut self, name: &'a str) -> App<'a> {
        self.name = Some(name);
        self
    }

    /// Set version
    pub fn version(mut self, version: &'a str) -> App<'a> {
        self.version = Some(version);
        self
    }

    /// Set name and version
    pub fn config(mut self, name: &'a str, version: &'a str) -> App<'a> {
        self.name = Some(name);
        self.version = Some(version);
        self
    }

    /// Set desc
    pub fn desc(mut self, desc: &'a str) -> App<'a> {
        self.desc = Some(desc);
        self
    }

    /// Add a command
    pub fn cmd<T: AppDesc>(mut self, cmd: &'a str, desc: T) -> App<'a> {
        self.command.push((cmd, desc.to_vec_string()));
        self
    }

    /// Add a option
    pub fn opt<T: AppDesc>(mut self, opt: &'a str, desc: T) -> App<'a> {
        self.option.push((opt, desc.to_vec_string()));
        self
    }

    /// Get the current command
    pub fn command(&self) -> Option<&String> {
        if let Some(cur) = self.args.get(1) {
            let all = self.option.iter().all(|(item, _)| item != cur);
            if all {
                return Some(cur);
            }
        }
        None
    }

    /// Get the value of command or option
    pub fn value(&self, name: &str) -> Option<Vec<&String>> {
        let find = self.args().iter().position(|item| item == name);

        if let Some(i) = find {
            let mut values = vec![];

            for item in &self.args()[i + 1..] {
                let pass = self.option.iter().all(|(arg, _)| item != arg);
                if pass {
                    values.push(item);
                } else {
                    break;
                }
            }

            return Some(values);
        }
        None
    }

    /// Match the current command
    pub fn is(&mut self, arg: &str) -> bool {
        self.args.len() > 1 && arg == self.args[1]
    }

    /// Get all values
    pub fn args(&self) -> &[String] {
        &self.args[1..]
    }

    /// Print version information
    pub fn print_version(&self) {
        if let (Some(name), Some(version)) = (self.name, self.version) {
            println!("{} version {}", name, version);
        } else if let Some(version) = self.version {
            println!("version {}", version);
        }
    }

    /// Print error information
    pub fn print_error(&self) {
        match self.args.get(1) {
            Some(cmd) => eprintln!("error: '{}' is not a valid command", cmd),
            None => eprintln!("error: valid command"),
        }
    }

    /// Print an error message and add an attempt
    pub fn print_error_try(&self, cmd: &str) {
        self.print_error();
        eprintln!("try:\n    '{} {}'", &self.args[0], cmd);
    }

    fn print_item(name: &'static str, data: &[(&str, Vec<String>)]) {
        println!("{}", name);
        let n = data
            .iter()
            .map(|item| item.0.len())
            .fold(0, |a, b| a.max(b));

        for (arg, desc) in data {
            println!(
                "    {:arg$}    {}",
                arg,
                desc.get(0).unwrap_or(&String::new()),
                arg = n
            );
            if desc.len() > 1 {
                for item in &desc[1..] {
                    if !item.is_empty() {
                        println!("{:>width$}", item, width = 8 + n + item.len());
                    }
                }
            }
        }
    }

    /// Print help information
    pub fn print_help(&self) {
        let name = self.name.map(|s| format!("{} ", s)).unwrap_or_default();
        let version = self
            .version
            .map(|s| format!("version {}", s))
            .unwrap_or_default();

        if !name.is_empty() || !version.is_empty() {
            println!("{}{}\n", name, version);
        }

        if let Some(desc) = self.desc {
            println!("{}\n", desc);
        }

        println!(
            "\
Usage:
    {} [COMMAND] [OPTION]
            ",
            self.args[0]
        );

        if !self.command.is_empty() {
            Self::print_item("Command:", &self.command);
        }

        if !self.command.is_empty() && !self.option.is_empty() {
            println!();
        }

        if !self.option.is_empty() {
            Self::print_item("Option:", &self.option);
        }
    }
}

pub trait AppDesc {
    fn to_vec_string(self) -> Vec<String>;
}

impl AppDesc for &str {
    fn to_vec_string(self) -> Vec<String> {
        vec![self.to_string()]
    }
}

impl AppDesc for String {
    fn to_vec_string(self) -> Vec<String> {
        vec![self]
    }
}

impl<T: ToString> AppDesc for &[T] {
    fn to_vec_string(self) -> Vec<String> {
        self.iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
    }
}

impl<T: ToString> AppDesc for Vec<T> {
    fn to_vec_string(self) -> Vec<String> {
        self.iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
    }
}
