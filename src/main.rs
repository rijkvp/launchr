mod app;
mod component;
mod mode;
mod render;

use app::App;
use clap::Parser;
use mode::{DmenuMode, FileMode, Mode, RunMode};
use std::io::{self, Read};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// dmenu mode
    #[arg(short, long)]
    dmenu: bool,
    /// Mode to use
    #[arg(short, long, default_value = "run")]
    mode: String,
}

fn main() {
    let args: Args = Args::parse();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let mode: Box<dyn Mode> = if args.dmenu {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from stdin");
        Box::new(DmenuMode::new(buffer))
    } else {
        match args.mode.as_str() {
            "run" => Box::new(RunMode),
            "file" => Box::new(FileMode::new(dirs::home_dir().unwrap())),
            other => {
                eprintln!("Unknown mode: {}", other);
                std::process::exit(1);
            }
        }
    };
    App::new(mode).run();
}
