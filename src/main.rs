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
    let mut gpio = gpio.unwrap();
    for i in 17..18 {
        gpio.fsel(i, bcm2835_rs::FunctionSelect::Outp);
        gpio.write(i, bcm2835_rs::PinVoltage::High);
        gpio.fsel(i, bcm2835_rs::FunctionSelect::Inpt);
        log::debug!("Pin state: {}={:?}", i, gpio.read(i));
    }
}
