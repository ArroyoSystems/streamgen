use crate::writers::GenWriter;
use async_trait::async_trait;
use clap::Args;
use std::io::Write;

pub struct StdoutWriter {
    stdout: std::io::Stdout,
}

#[derive(Clone, Debug, Args)]
pub struct StdoutConfig {}

impl StdoutConfig {
    pub fn to_writer(&self) -> Box<dyn GenWriter> {
        Box::new(StdoutWriter::new())
    }
}

impl StdoutWriter {
    pub fn new() -> Self {
        StdoutWriter {
            stdout: std::io::stdout(),
        }
    }
}

#[async_trait]
impl GenWriter for StdoutWriter {
    async fn write(&mut self, data: Vec<u8>) {
        self.stdout.write_all(&data).unwrap();
        write!(self.stdout, "\n").unwrap();
    }
}
