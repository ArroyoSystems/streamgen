use crate::writers::GenWriter;
use async_trait::async_trait;
use axum::extract::State;
use axum::response::sse::{Event, KeepAlive};
use axum::response::Sse;
use axum::routing::get;
use axum::Router;
use clap::Args;
use http::{HeaderMap, HeaderName};
use std::collections::BTreeMap;
use std::convert::Infallible;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::mpsc::Receiver;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::{Stream, StreamExt};
use tracing::info;

const MAX_HISTORY: usize = 10_000;

pub struct SSEServer {
    event_stream: Receiver<Vec<u8>>,
}

#[derive(Clone, Debug, Args)]
pub struct SSEConfig {
    /// Port for SSE server
    #[arg(long)]
    port: Option<u16>,
}

impl SSEConfig {
    pub fn to_writer(&self) -> Box<dyn GenWriter> {
        Box::new(SSEWriter::new(self.port.unwrap_or(9563)))
    }
}

#[derive(Clone)]
pub struct ServerState {
    events: Arc<tokio::sync::Mutex<BTreeMap<usize, Vec<u8>>>>,
    rx: Arc<broadcast::Receiver<(usize, Vec<u8>)>>,
}

impl SSEServer {
    pub async fn start(mut self, port: u16) {
        let events = Arc::new(tokio::sync::Mutex::new(BTreeMap::new()));
        let (tx, rx) = broadcast::channel(10);

        {
            let events = events.clone();
            let mut counter = 0;
            tokio::spawn(async move {
                while let Some(data) = self.event_stream.recv().await {
                    {
                        let mut events = events.lock().await;
                        events.insert(counter, data.clone());
                        counter += 1;
                        if counter > MAX_HISTORY {
                            events.remove(&(counter - MAX_HISTORY));
                        }
                    }

                    tx.send((counter, data)).unwrap();
                }
            });
        }

        let state = ServerState {
            events,
            rx: Arc::new(rx),
        };

        let app = Router::new()
            .route("/sse", get(sse_handler))
            .with_state(state);

        // run it
        let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
            .await
            .unwrap();
        info!("listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, app).await.unwrap();
    }
}

async fn sse_handler(
    headers: HeaderMap,
    State(state): State<ServerState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let last_event: Option<usize> = headers
        .get(HeaderName::from_str("Last-Event-ID").unwrap())
        .iter()
        .flat_map(|hv| hv.to_str().ok())
        .flat_map(|v| v.parse::<usize>().ok())
        .next();

    let mut rx: broadcast::Receiver<(usize, Vec<u8>)> = state.rx.resubscribe();
    let (id, _) = rx.recv().await.unwrap();

    let backfill: Vec<(usize, Vec<u8>)> = if let Some(last_event) = last_event {
        let events = state.events.lock().await;
        if events.contains_key(&last_event) {
            events
                .range(last_event..id)
                .map(|(id, record)| (*id, record.clone()))
                .collect()
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    let stream = tokio_stream::iter(backfill.into_iter())
        .chain(BroadcastStream::new(rx).map(|t| t.unwrap()))
        .map(|(id, message)| {
            Ok(Event::default()
                .id(id.to_string())
                .data(String::from_utf8(message).unwrap()))
        });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

pub struct SSEWriter {
    tx: tokio::sync::mpsc::Sender<Vec<u8>>,
}

impl SSEWriter {
    pub fn new(port: u16) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(1000);

        let server = SSEServer { event_stream: rx };

        tokio::spawn(async move {
            server.start(port).await;
        });

        SSEWriter { tx }
    }
}

#[async_trait]
impl GenWriter for SSEWriter {
    async fn write(&mut self, data: Vec<u8>) {
        self.tx.send(data).await.unwrap();
    }
}
