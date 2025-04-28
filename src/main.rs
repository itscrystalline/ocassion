use ocassion::config::Config;

fn main() {
    let Ok(config) = Config::load_or_default() else {
        return;
    };
    println!("{}", ocassion::output_of(config));
}
