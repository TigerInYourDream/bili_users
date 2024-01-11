pub fn init_log() {
    simple_logger::SimpleLogger::new().env().init().unwrap();
}
