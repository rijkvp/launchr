mod apps;
mod dmenu;
mod files;
mod run;

use std::{
    fmt::{self, Display, Formatter},
    sync::{Arc, Mutex},
    thread,
};

pub use apps::AppsMode;
pub use dmenu::DmenuMode;
pub use files::*;
pub use run::RunMode;

use crate::{
    item::{Action, Item},
    winit_app::EventHandle,
};
use nucleo_matcher::{
    pattern::{CaseMatching, Normalization, Pattern},
    Config, Matcher,
};

pub trait Mode {
    fn name(&self) -> &str;
    fn run(&self, event_handle: EventHandle);
    fn update(&mut self, input: &str) -> Vec<Match>;
}

pub trait SimpleMode {
    fn name(&self) -> &str;
    fn get_items(&mut self) -> &Vec<Item>;
}

impl<T: SimpleMode> Mode for T {
    fn name(&self) -> &str {
        self.name()
    }

    fn run(&self, _: EventHandle) {}

    fn update(&mut self, input: &str) -> Vec<Match> {
        let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
        Pattern::parse(input, CaseMatching::Ignore, Normalization::Smart)
            .match_list(self.get_items(), &mut matcher)
            .into_iter()
            .map(|(item, score)| Match {
                item: item.clone(),
                score: score as u64,
            })
            .take(64) // Limit the results
            .collect()
    }
}

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

    pub fn exec(self) {
        self.item.exec();
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

#[derive(Default)]
pub struct TestMode {
    items: Arc<Mutex<Vec<Item>>>,
}

impl Mode for TestMode {
    fn name(&self) -> &str {
        "Multithreading Test"
    }

    fn run(&self, event_handle: EventHandle) {
        let items = self.items.clone();
        thread::spawn(move || {
            let mut count = 0;
            loop {
                thread::sleep(std::time::Duration::from_millis(1000));
                items
                    .lock()
                    .unwrap()
                    .push(Item::new(format!("test item {}", count), Action::Selection));
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
