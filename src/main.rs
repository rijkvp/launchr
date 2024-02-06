mod app;
mod editor;
mod mode;
mod render;
mod text;

use app::App;

fn main() {
    App::new().run();
}
