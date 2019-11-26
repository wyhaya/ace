/// ## Initialize command line parsing

/// ### Example
/// ```rust
/// use ace::App;
///
/// let app = App::new("app", env!("CARGO_PKG_VERSION"))
///     .cmd("start", "Start now")
///     .cmd("help", "Display help information")
///     .cmd("version", "Display version information")
///     .opt("--config", "Use configuration file");
///
/// if let Some(cmd) = app.command() {
///     match cmd.as_str() {
///         "start" => {
///             dbg!(app.value("--config"));
///         }
///         "help" => {
///             app.help();
///         }
///         "version" => {
///             app.version();
///         }
///         _ => {
///             app.error_try("help");
///         }
///     }
/// }
/// ```

#[derive(Debug, Clone)]
pub struct App<'a> {
    name: &'a str,
    version: &'a str,
    args: Vec<String>,
    command: Vec<(&'a str, &'a str)>,
    option: Vec<(&'a str, &'a str)>,
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
    pub fn cmd(mut self, cmd: &'a str, desc: &'a str) -> App<'a> {
        self.command.push((cmd, desc));
        App { ..self }
    }

    /// Add a option
    pub fn opt(mut self, opt: &'a str, desc: &'a str) -> App<'a> {
        self.option.push((opt, desc));
        App { ..self }
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

    /// Match the current command
    pub fn is(&mut self, arg: &str) -> bool {
        self.args.len() > 1 && arg == self.args[1]
    }

    // Get all values
    pub fn values(&self) -> &[String] {
        &self.args[1..]
    }

    /// Get the value of option
    pub fn value(&self, option: &str) -> Option<Vec<&String>> {
        let mut values = vec![];
        let mut find = false;
        for item in self.args[1..].iter() {
            if find {
                let all = self.option.iter().all(|(arg, _)| item != arg);
                if all {
                    values.push(item);
                } else {
                    break;
                }
            }
            if item == option {
                find = true;
            }
        }
        if find {
            Some(values)
        } else {
            None
        }
    }

    /// Print version information
    pub fn version(&self) {
        println!("{0} version {1}", self.name, self.version)
    }

    fn print_help(name: &'static str, data: &[(&str, &str)]) {
        println!("{}", name);
        let n = data
            .iter()
            .map(|item| item.0.len())
            .fold(0, |a, b| a.max(b));

        for (arg, desc) in data {
            println!("    {:arg$}    {}", arg, desc, arg = n);
        }
    }

    /// Print help information
    pub fn help(&self) {
        println!(
            "\
{0} version {1}

Usage:
    {0} [COMMAND] [OPTION]
            ",
            self.name, self.version
        );

        if !self.command.is_empty() {
            Self::print_help("Command:", &self.command);
        }

        if !self.command.is_empty() && !self.option.is_empty() {
            println!();
        }

        if !self.option.is_empty() {
            Self::print_help("Option:", &self.option);
        }
    }

    /// Print error information
    pub fn error(&self) {
        eprint!("\x1B[1;31m{}\x1B[0m", "error: ");
        eprintln!(
            "'{}' is not a valid command",
            self.args.get(1).unwrap_or(&String::with_capacity(0))
        );
    }

    /// Print an error message and add an attempt
    pub fn error_try(&self, command: &str) {
        self.error();
        eprintln!("try:\n    '{} {}'", self.name, command);
    }
}
