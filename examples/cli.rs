use ace::App;

fn main() {
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
    } else {
        dbg!(app.values());
    }
}
