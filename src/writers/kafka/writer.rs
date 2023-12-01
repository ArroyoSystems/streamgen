use crate::writers::kafka::KafkaConfig;
use crate::writers::GenWriter;
use async_trait::async_trait;
use rdkafka::error::{KafkaError, RDKafkaErrorCode};
use rdkafka::producer::{DeliveryFuture, FutureProducer};
use std::time::Duration;
use tracing::info;
use uuid::Uuid;

pub struct KafkaWriter {
    topic: String,
    future_producer: FutureProducer,
}

impl KafkaWriter {
    pub fn new(config: &KafkaConfig) -> Self {
        let mut client_config = rdkafka::ClientConfig::new();
        client_config.set("bootstrap.servers", &config.bootstrap_servers);

        if let Some(options) = &config.options {
            for option in options {
                let parts: Vec<&str> = option.splitn(2, '=').collect();
                if parts.len() != 2 {
                    panic!("invalid option: {}", option);
                }
                client_config.set(parts[0].to_string(), parts[1].to_string());
            }
        }

        let producer: FutureProducer = client_config.create().unwrap();

        info!("Writing to kafka topic: {}...", config.topic);

        KafkaWriter {
            topic: config.topic.clone(),
            future_producer: producer,
        }
    }

    async fn publish(&mut self, data: Vec<u8>) -> DeliveryFuture {
        let key = Uuid::new_v4().to_string();
        let mut record = rdkafka::producer::FutureRecord::to(&self.topic)
            .key(key.as_bytes())
            .payload(&data);

        loop {
            match self.future_producer.send_result(record) {
                Ok(future) => {
                    return future;
                }
                Err((KafkaError::MessageProduction(RDKafkaErrorCode::QueueFull), f)) => {
                    record = f;
                }
                Err((e, _)) => {
                    panic!("Unhandled kafka error: {:?}", e);
                }
            }

            // back off and retry
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }
}

#[async_trait]
impl GenWriter for KafkaWriter {
    async fn write(&mut self, data: Vec<u8>) {
        let future = self.publish(data).await;

        tokio::spawn(async move {
            match future.await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error publishing to kafka: {:?}", e);
                }
            }
        });
    }
}
