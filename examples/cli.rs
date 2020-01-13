use ace::App;

fn main() {
    let app = App::new("app", env!("CARGO_PKG_VERSION"))
        .cmd("start", "Start now")
        .cmd("help", "Display help information")
        .cmd("version", "Display version information")
        .opt("--config", "Use configuration file")
        .opt(
            "--duration",
            vec!["Set duration of test", "example (1ms, 1s, 1m, 1h, 1d)"],
        )
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
