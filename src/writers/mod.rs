use async_trait::async_trait;

pub mod kafka;
pub mod sse;
pub mod stdout;

#[async_trait]
pub trait GenWriter {
    async fn write(&mut self, data: Vec<u8>);
}
