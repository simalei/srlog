# srlog

Simple logging with templating

- **Disclaimer**: *srlog was created as an alternative to the **spdlog** in Rust. However, since I'm still a beginner, the code may be bad and so on. Any contribution or feature suggestion is much appreciated!*

Use `cargo add srlog` to add srlog to your project

# Examples

- Basic usage
```rust

use srlog::logger::Logger;

fn main() {
    let mut logger = Logger::new();
    logger.debug("debug message");
    logger.info("info message");
    logger.warn("warning message");
    logger.error("error message");
}


```

- Custom templates
```rust
use srlog::logger::Logger;

fn main() {
    let mut logger = Logger::new();
    logger.info("This is default template");
    logger.set_template("{level} {message}").expect("Failed to set template");
    logger.info("This is new template");
}

```
# Supported keys


| Key         | Output      |
| ----------- | ----------- |
| `level`       | Level of the log message |
| `message`     | Message itself       |
| `hour`        | Hour                 |
| `day`         | Day                  |
| `month_num`   | Month as a number    |
| `month_name`  | Name of the month    |
| `year`        | Year                 |
| `time`        | Time                 |
| `date`        | Date                 |
