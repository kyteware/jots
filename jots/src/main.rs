mod app;
mod error;
mod fs;
mod sidebar;

use app::App;
use iced::{Application, Settings};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::prep_data_dir().await?;
    App::run(Settings::default())?;
    Ok(())
}
