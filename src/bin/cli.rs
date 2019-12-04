#[macro_use]
extern crate log;
extern crate clap;

fn main() {
    env_logger::Builder::from_default_env()
        .parse_filters("info")
        .init();

    info!("Hello, World!");
}
