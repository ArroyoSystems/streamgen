use crate::generators::Generator;
use crate::Format;
use chrono::{DateTime, Local};
use fake::faker::filesystem::en::FilePath;
use fake::faker::internet::en::{UserAgent, Username, IP};
use fake::Fake;
use rand::prelude::IteratorRandom;
use rand::rngs::ThreadRng;
use rand::Rng;
use serde::Serialize;
use std::fmt::Display;

pub struct CommonLogGenerator {
    format: Format,
    rand: ThreadRng,
}

const ATTACKER: &str = "34.127.44.91";

#[derive(Debug, Serialize)]
pub struct CommonLog {
    ip: String,
    identity: String,
    user_id: String,
    timestamp: DateTime<Local>,
    request: String,
    status_code: u16,
    size: u32,
    referer: String,
    user_agent: String,
}

const HTTP_METHODS: [&str; 4] = ["GET", "POST", "PUT", "DELETE"];

fn get_method(rng: &mut ThreadRng) -> &'static str {
    for method in HTTP_METHODS {
        if rng.gen_bool(0.9) {
            return method;
        }
    }

    HTTP_METHODS.last().unwrap()
}

const STATUS_CODES: [u16; 7] = [200u16, 400, 401, 403, 404, 405, 500];

fn get_status_code(rng: &mut ThreadRng) -> http::StatusCode {
    http::StatusCode::from_u16(*STATUS_CODES.iter().choose(rng).unwrap()).unwrap()
}

impl CommonLog {
    fn rand(rng: &mut ThreadRng) -> Self {
        let (ip, status_code) = if rng.gen_bool(0.05) {
            (ATTACKER.to_string(), http::StatusCode::UNAUTHORIZED)
        } else {
            (IP().fake_with_rng(rng), get_status_code(rng))
        };

        CommonLog {
            ip,
            identity: "-".to_string(),
            user_id: Username().fake_with_rng(rng),
            timestamp: Local::now(),
            request: format!(
                "{} {}",
                get_method(rng),
                FilePath().fake_with_rng::<String, _>(rng)
            ),
            status_code: status_code.as_u16(),
            size: rng.gen_range(500..10000),
            referer: "-".to_string(),
            user_agent: UserAgent().fake_with_rng(rng),
        }
    }
}

impl Display for CommonLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} [{}] \"{}\" {} {} \"{}\" \"{}\"",
            self.ip,
            self.identity,
            self.user_id,
            self.timestamp.format("%d/%b/%Y:%H:%M:%S %z"),
            self.request,
            self.status_code,
            self.size,
            self.referer,
            self.user_agent,
        )
    }
}

impl CommonLogGenerator {
    pub fn new(format: Format) -> CommonLogGenerator {
        CommonLogGenerator {
            format: format,
            rand: rand::thread_rng(),
        }
    }
}

impl Generator for CommonLogGenerator {
    fn generate(&mut self) -> Vec<u8> {
        let log = CommonLog::rand(&mut self.rand);
        match self.format {
            Format::String => log.to_string().into_bytes(),
            Format::Json => serde_json::to_vec(&log).unwrap(),
        }
    }
}
