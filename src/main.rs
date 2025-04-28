use ocassion::config::Config;

fn main() {
    let Ok(config) = Config::load_or_default() else {
        return;
    };
    for message in config.dates {
        let Some(msg) = message.try_message() else {
            continue;
        };
        print!("{msg}");
    }
}
