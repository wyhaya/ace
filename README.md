
# Ace

[![Build Status](https://img.shields.io/travis/wyhaya/ace.svg?style=flat-square)](https://travis-ci.org/wyhaya/ace)
[![Crates.io](https://img.shields.io/crates/v/ace.svg?style=flat-square)](https://crates.io/crates/ace)
[![LICENSE](https://img.shields.io/crates/l/ace.svg?style=flat-square)](https://crates.io/crates/ace)
 
A simple command line parameter parsing library
 
 ## Install

Add this in your `Cargo.toml`:

```toml
[dependencies]
ace = "0.0.2"
```

 ## Example
 
```rust
use ace::App;

let app = App::new("ace", env!("CARGO_PKG_VERSION"))
    .cmd("start", "Start now")
    .cmd("help", "Display help information")
    .cmd("version", "Display version information")
    .opt("--config", "Use configuration file");

if let Some(cmd) = app.command() {
    match cmd.as_str() {
        "start" => {
            dbg!(app.value("--config"));
        }
        "help" => {
            app.help();
        }
        "version" => {
            app.version();
        }
        _ => {
            app.error_try("help");
        }
    }
}
```