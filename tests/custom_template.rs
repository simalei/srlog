use srlog::logger::Logger;

fn main() {
    let mut logger = Logger::new();
    logger.info("This is default template");
    logger.set_template("{level} {message}").expect("Failed to set template");
    logger.info("This is new template");
}