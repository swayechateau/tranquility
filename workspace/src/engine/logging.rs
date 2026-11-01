//! File logging for `tquil`, read back by the `logs` command.
//!
//! Writes one JSON object per line to `{log_directory}/{YYYY-MM-DD}-tranquility.log`,
//! rolling over automatically at local midnight. Kept deliberately simple (reopen
//! the file on every write) since CLI invocations are short-lived and low-volume.

use std::{io, path::PathBuf};

/// Writes to today's dated log file under a fixed directory, matching the
/// filename `tquil logs` expects (`{date}-tranquility.log`).
#[derive(Clone)]
pub struct DailyFileWriter {
    dir: PathBuf,
}

impl DailyFileWriter {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    fn today_path(&self) -> PathBuf {
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        self.dir.join(format!("{date}-tranquility.log"))
    }
}

impl io::Write for DailyFileWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        std::fs::create_dir_all(&self.dir)?;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.today_path())?;
        file.write_all(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for DailyFileWriter {
    type Writer = DailyFileWriter;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}
