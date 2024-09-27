mod apps;
mod dmenu;
mod files;
mod run;

use std::process::Command;

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
    pub item: Item,
    pub score: u32,
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
                score,
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
