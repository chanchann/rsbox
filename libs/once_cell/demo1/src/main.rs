use std::{env, io};

use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct Logger {
    level: String,
}

static INSTANCE: OnceCell<Logger> = OnceCell::new();

impl Logger {
    pub fn global() -> &'static Logger {
        INSTANCE.get().expect("logger is not initialized")
    }

    fn from_cli(mut args: env::Args) -> Result<Logger, std::io::Error> {
        args.next(); // Skip the executable name
        let level = match args.next() {
            Some(level) => level,
            None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "missing log level")),
        };

        Ok(Logger { level })
    }

    pub fn log(&self, message: &str) {
        println!("[{}] {}", self.level, message);
    }
}

fn main() {
    // cargo run info
    let logger = Logger::from_cli(env::args()).unwrap();
    INSTANCE.set(logger).unwrap();

    // Use Logger::global() from now on
    Logger::global().log("Hello, world!");
}