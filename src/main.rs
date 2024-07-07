use launcher::{app::App, winit_app::launch_winit_app};

fn main() {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();

    launch_winit_app::<App>();
}
