pub mod app;
pub mod goal_tree;

use app::App;

fn main() {
    iced::application("Jots", App::update, App::view)
        .run_with(|| App::new())
        .unwrap();
}
