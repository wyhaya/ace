use ace::Ace;

fn main() {
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
}
