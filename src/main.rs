fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::Builder::from_default_env()
        .format_timestamp_millis()
        .init();

    let gpio = bcm2835_rs::Bcm2835Gpio::new();
    if gpio.is_none() {
        log::error!("Failed to initialize chip");
    }
    let gpio = gpio.unwrap();
    log::debug!("Pin state: {:?}", gpio.read(18));
}
