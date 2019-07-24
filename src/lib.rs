#[derive(Debug, Clone)]
pub struct Ace {
    args: Vec<String>,
    options: Vec<(String, String)>,
}

impl Ace {
    pub fn new() -> Ace {
        let args = std::env::args().collect::<Vec<String>>();
        Ace {
            args,
            options: vec![],
        }
    }

    pub fn arg(mut self, arg: &str, desc: &str) -> Ace {
        self.options.push((arg.to_string(), desc.to_string()));
        Ace { ..self }
    }

    pub fn is(&mut self, arg: &str) -> bool {
        self.args.len() > 1 && arg == self.args[1]
    }

    pub fn command(&self) -> Option<&String> {
        self.args.get(1)
    }

    pub fn value(&self) -> Vec<String> {
        if self.args.len() > 2 {
            self.args[2..].to_vec()
        } else {
            vec![]
        }
    }

    pub fn value_from(&self, command: &str) -> Vec<String> {
        let mut values = vec![];
        let mut f = false;
        self.args[1..].iter().for_each(|s| {
            if f {
                values.push(s.to_string());
            }
            if s == command {
                f = true;
            }
        });
        values
    }

    pub fn version(&self) {
        println!(
            "{0} version {1}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )
    }

    pub fn help(&self) {
        println!(
            "\
{0} version {1}

Usage:
    {0} [COMMAND] [OPTION]

Command:\
            ",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
        let mut n = 0;
        self.options.iter().for_each(|item| {
            if item.0.len() > n {
                n = item.0.len();
            }
        });
        for (arg, desc) in &self.options {
            println!("    {:arg$}    {}", arg, desc, arg = n);
        }
    }

    pub fn error(&self) {
        eprint!("\x1B[1;31m{}\x1B[0m", "error: ");
        eprintln!(
            "'{}' is not a valid command",
            self.args.get(1).unwrap_or(&String::new())
        );
    }

    pub fn error_try(&self, command: &str) {
        self.error();
        eprintln!("try:\n    '{} {}'", env!("CARGO_PKG_NAME"), command);
    }
}
