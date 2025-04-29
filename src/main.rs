use occasion::config::Config;

fn main() {
    let Ok(config) = Config::load_or_default() else {
        return;
    };
    println!("{}", occasion::output_of(&config));
}
