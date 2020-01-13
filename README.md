
# Ace
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/wyhaya/ace/Test?style=flat-square)](https://github.com/wyhaya/ace/actions)
[![Crates.io](https://img.shields.io/crates/v/ace.svg?style=flat-square)](https://crates.io/crates/ace)
[![LICENSE](https://img.shields.io/crates/l/ace.svg?style=flat-square)](https://crates.io/crates/ace)
 
A simple command line parameter parsing library
 
 ## Install

Add this in your `Cargo.toml`:

```toml
[dependencies]
ace = "0.1.0"
```

## Example
 
```rust
use ace::App;

fn main() {
    let app = App::new("app", env!("CARGO_PKG_VERSION"))
        .cmd("start", "Start now")
        .cmd("help", "Display help information")
        .cmd("version", "Display version information")
        .opt("--config", "Use configuration file")
        .opt("--duration", vec!["Set duration of test", "example (1ms, 1s, 1m, 1h, 1d)"])
        .opt("--timeout", "Set timeout");

    if let Some(cmd) = app.command() {
        match cmd.as_str() {
            "start" => {
                dbg!(app.value("--config"));
            }
            "help" => {
                app.print_help();
            }
            "version" => {
                app.print_version();
            }
            _ => {
                app.print_error_try("help");
            }
        }
    } else {
        dbg!(app.args());
    }
}

```

Output:

```bash
app version 0.1.0

Usage:
    app [COMMAND] [OPTION]
            
Command:
    start      Start now
    help       Display help information
    version    Display version information

Option:
    --config      Use configuration file
    --duration    Set duration of test
                  example (1ms, 1s, 1m, 1h, 1d)
    --timeout     Set timeout
```