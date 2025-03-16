use clap::Parser;
use launchr::{
    launcher::Launcher,
    mode::{AppsMode, DmenuMode, FilesMode, Mode, RunMode},
    winit_app::WinitApp,
};
use std::io::{Read, stdin};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// dmenu mode
    #[arg(short, long)]
    dmenu: bool,
    /// Prompt to display in dmenu mode
    #[arg(short, long)]
    prompt: Option<String>,
    /// Mode to use
    #[arg(short, long, default_value = "run")]
    mode: String,
}

fn main() {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();

    let args: Args = Args::parse();
    let mode: Box<dyn Mode> = if args.dmenu {
        let mut buffer = String::new();
        stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from stdin");
        Box::new(DmenuMode::new(args.prompt, buffer))
    } else {
        match args.mode.as_str() {
            "apps" => Box::new(AppsMode::load()),
            "run" => Box::new(RunMode::load()),
            "files" => Box::new(FilesMode::new(dirs::home_dir().unwrap())),
            other => {
                eprintln!("Unknown mode: {}", other);
                std::process::exit(1);
            }
        }
    };
    match Launcher::load(mode) {
        Ok(launcher) => WinitApp::new(launcher).run(),
        Err(e) => {
            eprintln!("Failed to load launcher: {e}");
            std::process::exit(1);
        }
    }
}
