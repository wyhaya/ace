
# Ace

[![Build Status](https://img.shields.io/travis/wyhaya/ace.svg?style=flat-square)](https://travis-ci.org/wyhaya/ace)
[![Crates.io](https://img.shields.io/crates/v/ace.svg?style=flat-square)](https://crates.io/crates/ace)
[![LICENSE](https://img.shields.io/crates/l/ace.svg?style=flat-square)](https://crates.io/crates/ace)
 
A simple command line parameter parsing library
 
 ## Install

Add this in your `Cargo.toml`:

```toml
[dependencies]
ace = "0.0.1"
```

 ## Example
 
```rust
use ace::Ace;

let app = Ace::new()
    .arg("start", "Start daemon")
    .arg("help", "Display help information")
    .arg("version", "Display version information");

if let Some(cmd) = app.command() {
    match cmd.as_str() {
        "start" => {
            dbg!(app.value());
        }
        "help" => {
            app.help();
        }
        "version" => {
            app.version();
        }
        _ => {
            app.error();
        }
    }
} else {
    app.error_try("help");
}
```