mod generators;
mod writers;

use crate::generators::schematized::{OrderRecord, StockTrade};
use crate::generators::Generator;
use crate::writers::kafka::KafkaConfig;
use crate::writers::sse::SSEConfig;
use crate::writers::stdout::StdoutConfig;
use crate::writers::GenWriter;
use clap::{Parser, Subcommand, ValueEnum};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

const DEFAULT_RATE: f32 = 50.0;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Format {
    String,
    Json,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum GeneratorSpec {
    CommonLog,
    Impulse,
    Order,
    StockTrade,
    Nexmark,
}

impl GeneratorSpec {
    pub fn generator(&self, format: Format, event_rate: f32) -> Box<dyn Generator> {
        match self {
            GeneratorSpec::CommonLog => {
                Box::new(generators::common_log::CommonLogGenerator::new(format))
            }
            GeneratorSpec::Impulse => Box::new(generators::ImpulseGen::new(format)),
            GeneratorSpec::Order => {
                Box::new(generators::schematized::SchemaGenerator::<OrderRecord>::new(format))
            }
            GeneratorSpec::StockTrade => Box::new(generators::schematized::SchemaGenerator::<
                StockTrade,
            >::new(format)),
            GeneratorSpec::Nexmark => Box::new(generators::nexmark::NexmarkGen::new(format, event_rate as f64))
        }
    }
}

/// Generate realistic streaming data for testing stream processing applications
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Type of data to generator
    spec: GeneratorSpec,

    /// Format of the generated data
    #[arg(short, long)]
    format: Option<Format>,

    /// Rate of generation in records per second
    #[arg(short, long)]
    rate: Option<f32>,

    /// Max number of records to generate
    #[arg(short, long)]
    limit: Option<usize>,

    /// Controls where the generated data is sent
    #[command(subcommand)]
    output: Option<OutputCommand>,
}

#[derive(Clone, Debug, Subcommand)]
pub enum OutputCommand {
    /// Write outputs to stdout
    Stdout(StdoutConfig),
    /// Run a Server-Sent Events server
    SSE(SSEConfig),
    /// Write outputs to Kafka
    Kafka(KafkaConfig),
}

impl OutputCommand {
    pub fn writer(&self) -> Box<dyn GenWriter> {
        match self {
            OutputCommand::Stdout(config) => config.to_writer(),
            OutputCommand::SSE(config) => config.to_writer(),
            OutputCommand::Kafka(config) => config.to_writer(),
        }
    }
}

#[tokio::main]
pub async fn main() {

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stderr))
        .init();

    let cli = Cli::parse();

    let mut generator = cli.spec.generator(cli.format.unwrap_or(Format::Json), 
                                           cli.rate.unwrap_or(DEFAULT_RATE));

    let mut writer = cli
        .output
        .unwrap_or(OutputCommand::Stdout(StdoutConfig {}))
        .writer();

    let mut count = 0;
    while count < cli.limit.unwrap_or(usize::MAX) {
        let data = generator.generate();
        writer.write(data).await;
        count += 1;

        tokio::time::sleep(std::time::Duration::from_secs_f32(
            1.0 / cli.rate.unwrap_or(DEFAULT_RATE),
        ))
        .await;
    }
}
