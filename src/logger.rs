use anyhow::Result;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub struct Logger {
    log_path: PathBuf,
}

impl Logger {
    pub fn new(log_dir: PathBuf) -> Result<Self> {
        fs::create_dir_all(&log_dir)?;

        Ok(Logger { log_path: log_dir })
    }

    pub fn get_log_file(&self, command: &str) -> PathBuf {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        self.log_path.join(format!("{command}_{timestamp}.log"))
    }

    pub fn log(&self, command: &str, message: &str) -> Result<()> {
        let log_file = self.get_log_file(command);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)?;

        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        writeln!(file, "[{timestamp}] {message}")?;

        Ok(())
    }

    pub fn log_section(&self, command: &str, title: &str) -> Result<()> {
        let log_file = self.get_log_file(command);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)?;

        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        writeln!(file, "\n[{timestamp}] === {title} ===")?;

        Ok(())
    }

    pub fn print_and_log(&self, command: &str, message: &str) -> Result<()> {
        println!("{message}");
        self.log(command, message)?;
        Ok(())
    }
}
