/// ## Initialize command line parsing

/// ### Example
/// ```rust
/// use ace::App;
///
/// let app = App::new("app", env!("CARGO_PKG_VERSION"))
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

#[derive(Debug, Clone)]
pub struct App<'a> {
    name: &'a str,
    version: &'a str,
    args: Vec<String>,
    command: Vec<(&'a str, Vec<String>)>,
    option: Vec<(&'a str, Vec<String>)>,
}

impl<'a> App<'a> {
    /// Create
    pub fn new(name: &'a str, version: &'a str) -> App<'a> {
        let args = std::env::args().collect::<Vec<String>>();
        App {
            name,
            version,
            args,
            command: vec![],
            option: vec![],
        }
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
        println!("{} version {}", self.name, self.version)
    }

    /// Print error information
    pub fn print_error(&self) {
        eprintln!(
            "error: '{}' is not a valid command",
            self.args.get(1).unwrap_or(&String::with_capacity(0))
        );
    }

    /// Print an error message and add an attempt
    pub fn print_error_try(&self, command: &str) {
        self.print_error();
        eprintln!("try:\n    '{} {}'", self.name, command);
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
        println!(
            "\
{0} version {1}

Usage:
    {0} [COMMAND] [OPTION]
            ",
            self.name, self.version
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

impl<T: ToString> AppDesc for Vec<T> {
    fn to_vec_string(self) -> Vec<String> {
        self.iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
    }
}
