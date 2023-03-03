pub mod error;
pub mod models;
pub mod queries;
pub mod utils;
pub mod dtos;

pub fn setup_tracing() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .without_time()
        .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        .init();
}
