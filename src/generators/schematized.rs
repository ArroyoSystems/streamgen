use crate::generators::Generator;
use crate::Format;
use chrono::Local;
use fake::faker::address::en::{CityName, StateAbbr, StreetName, ZipCode};
use fake::faker::internet::en::Username;
use fake::faker::name::en::Name;
use fake::{Fake, Faker};
use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use rand::Rng;
use serde::Serialize;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::OnceLock;

const SYMBOLS: &str = include_str!("../../resources/symbols.txt");

static SYMBOLS_LIST: OnceLock<Vec<&str>> = OnceLock::new();

pub trait Schema: Debug + Serialize {
    fn generate(rng: &mut ThreadRng) -> Self;
}

#[derive(Debug, Clone, Serialize)]
pub struct OrderRecord {
    pub order_id: u32,
    pub user_id: String,
    pub order_date: chrono::DateTime<Local>,
    pub total_amount: u64,

    pub name: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip: String,
}

impl Schema for OrderRecord {
    fn generate(rng: &mut ThreadRng) -> Self {
        OrderRecord {
            order_id: Faker.fake_with_rng(rng),
            user_id: Username().fake_with_rng(rng),
            order_date: Local::now(),
            total_amount: rng.gen_range(100..50000),
            name: Name().fake_with_rng(rng),
            address: format!(
                "{} {}",
                rng.gen_range(1..1000),
                StreetName().fake_with_rng::<String, _>(rng)
            ),
            city: CityName().fake_with_rng(rng),
            state: StateAbbr().fake_with_rng(rng),
            zip: ZipCode().fake_with_rng(rng),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct StockTrade {
    pub user_id: String,
    pub account_id: usize,
    pub symbol: String,
    pub side: String,
    pub unit_price: u64,
    pub units: u32,
    pub timestamp: chrono::DateTime<Local>,
}

impl Schema for StockTrade {
    fn generate(rng: &mut ThreadRng) -> Self {
        let symbols = SYMBOLS_LIST.get_or_init(|| SYMBOLS.lines().collect());

        StockTrade {
            user_id: Username().fake_with_rng(rng),
            account_id: rng.gen_range(1000..2000),
            symbol: symbols.choose(rng).unwrap().to_string(),
            side: if rng.gen_bool(0.5) { "buy" } else { "sell" }.to_string(),
            unit_price: rng.gen_range(1000..100000),
            units: rng.gen_range(1..100),
            timestamp: Local::now(),
        }
    }
}

pub struct SchemaGenerator<T: Schema> {
    rand: ThreadRng,
    format: Format,
    _t: PhantomData<T>,
}

impl<T: Schema> SchemaGenerator<T> {
    pub fn new(format: Format) -> Self {
        SchemaGenerator {
            format,
            rand: rand::thread_rng(),
            _t: PhantomData,
        }
    }
}

impl<T: Schema> Generator for SchemaGenerator<T> {
    fn generate(&mut self) -> Vec<u8> {
        let record = T::generate(&mut self.rand);

        match self.format {
            Format::String => format!("{:?}", record).into_bytes(),
            Format::Json => serde_json::to_vec(&record).unwrap(),
        }
    }
}
