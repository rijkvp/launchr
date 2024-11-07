mod apps;
mod dmenu;
mod files;
mod run;

use std::{
    fmt::{self, Display, Formatter},
    process::Command,
};

pub use apps::AppsMode;
pub use dmenu::DmenuMode;
pub use files::*;
pub use run::RunMode;

use crate::item::{Item, ItemType};
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
