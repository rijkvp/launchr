mod apps;
mod dmenu;
mod files;
mod run;

use std::{
    fmt::{self, Display, Formatter},
    process::Command,
    sync::{Arc, Mutex},
    thread,
};

pub use apps::AppsMode;
pub use dmenu::DmenuMode;
pub use files::*;
pub use run::RunMode;

use crate::{
    item::{Item, ItemType},
    winit_app::EventHandle,
};
use nucleo_matcher::{
    pattern::{CaseMatching, Normalization, Pattern},
    Config, Matcher,
};

pub struct Match {
    item: Item,
    score: u64,
}

impl Match {
    pub fn new(item: Item, score: u64) -> Self {
        Self { item, score }
    }

    pub fn item(&self) -> &Item {
        &self.item
    }

    pub fn score(&self) -> u64 {
        self.score
    }
}

impl Display for Match {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.item)?;
        if self.score > 0 {
            write!(f, " ({})", self.score)?;
        }
        Ok(())
    }
}

pub trait Mode {
    fn name(&self) -> &str;
    fn options(&mut self) -> &Vec<Item>;
    fn run(&mut self, input: &str) -> Vec<Match> {
        let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
        Pattern::parse(input, CaseMatching::Ignore, Normalization::Smart)
            .match_list(self.options(), &mut matcher)
            .into_iter()
            .map(|(item, score)| Match {
                item: item.clone(),
                score: score as u64,
            })
            .take(64) // Limit the results
            .collect()
    }

    fn exec(&self, item: &Item);
}

pub trait Mode2 {
    fn name(&self) -> &str;
    fn start(&self, event_handle: EventHandle);
    fn update(&mut self, input: &str) -> Vec<Match>;
}

#[derive(Default)]
pub struct TestMode {
    items: Arc<Mutex<Vec<Item>>>,
}

impl Mode2 for TestMode {
    fn name(&self) -> &str {
        "Multithreading Test"
    }

    fn start(&self, event_handle: EventHandle) {
        let items = self.items.clone();
        thread::spawn(move || {
            let mut count = 0;
            loop {
                thread::sleep(std::time::Duration::from_millis(10));
                items.lock().unwrap().push(Item::new(
                    format!("test item {}", count),
                    ItemType::Selection,
                ));
                log::info!("sending update");
                event_handle.send_update();
                count += 1;
            }
        });
    }

    fn update(&mut self, input: &str) -> Vec<Match> {
        log::info!("update: {}", input);
        let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
        let options = self.items.lock().unwrap().clone();
        Pattern::parse(input, CaseMatching::Ignore, Normalization::Smart)
            .match_list(options, &mut matcher)
            .into_iter()
            .map(|(item, score)| Match {
                item: item.clone(),
                score: score as u64,
            })
            .take(64) // Limit the results
            .collect()
    }
}

fn exec_item(item: &Item) {
    let exec = match item.item_type() {
        ItemType::Exec { exec } => exec,
        _ => panic!("expected exec item"),
    };
    // Execute the command as child process
    let cmd = exec.command();
    log::info!("executing: '{cmd}'");
    if let Err(e) = Command::new(&exec.program).args(&exec.args).spawn() {
        eprintln!("Failed to run '{cmd}': {e}");
    }
}
