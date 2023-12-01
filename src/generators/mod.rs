use crate::Format;
use serde::Serialize;

pub mod common_log;
pub mod schematized;

pub trait Generator {
    fn generate(&mut self) -> Vec<u8>;
}

pub struct ImpulseGen {
    counter: usize,
    format: Format,
}

impl ImpulseGen {
    pub fn new(format: Format) -> Self {
        ImpulseGen { counter: 0, format }
    }
}

#[derive(Serialize, Debug, Copy, Clone)]
struct ImpulseData {
    id: usize,
}

impl Generator for ImpulseGen {
    fn generate(&mut self) -> Vec<u8> {
        let data = ImpulseData { id: self.counter };
        self.counter += 1;

        match self.format {
            Format::String => format!("{}", data.id).into_bytes(),
            Format::Json => serde_json::to_vec(&data).unwrap(),
        }
    }
}
