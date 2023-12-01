#[cfg(feature = "kafka")]
mod writer;

use crate::writers::GenWriter;
use clap::Args;

#[derive(Clone, Debug, Args)]
pub struct KafkaConfig {
    /// Kafka bootstrap servers
    #[arg(long)]
    bootstrap_servers: String,

    /// Kafka topic
    #[arg(long)]
    topic: String,

    /// Set Kafka options as key=value pairs
    #[arg(long)]
    options: Option<Vec<String>>,
}

impl KafkaConfig {
    pub fn to_writer(&self) -> Box<dyn GenWriter> {
        #[cfg(feature = "kafka")]
        {
            Box::new(writer::KafkaWriter::new(self))
        }

        #[cfg(not(feature = "kafka"))]
        {
            panic!("streamgen was not compiled with Kafka support; recompile with --features kafka to use this writer")
        }
    }
}
