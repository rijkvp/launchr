mod apps;
mod dmenu;
mod files;
mod run;

pub use apps::AppsMode;
pub use dmenu::DmenuMode;
pub use files::FilesMode;
use nucleo_matcher::{
    pattern::{CaseMatching, Normalization, Pattern},
    Config, Matcher,
};
pub use run::RunMode;

use crate::item::Item;

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
}
