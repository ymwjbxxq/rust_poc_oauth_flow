pub mod dtos;
pub mod models;
pub mod queries;
pub mod utils;

pub fn setup_tracing() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .without_time()
        .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        .init();
}
