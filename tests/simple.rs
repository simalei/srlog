use srlog::logger::Logger;

fn main() {
    let mut logger = Logger::new();
    logger.debug("debug message");
    logger.info("info message");
    logger.warn("warning message");
    logger.error("error message");
}