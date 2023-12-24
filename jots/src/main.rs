mod app;
mod error;
mod fs;

use anyhow::Context;
use app::App;
use iced::{Application, Settings};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fs::prep_data_dir().await?;
    App::run(Settings::default()).context("Failed to run app")
}
