mod app;
mod editor;
mod render;
mod text;

use app::App;

fn main() {
    App::new().run();
}
