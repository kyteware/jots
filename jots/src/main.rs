mod app;
mod fs;

use app::App;
use iced::{Application, Result, Settings};

fn main() -> Result {
    App::run(Settings::default())
}
