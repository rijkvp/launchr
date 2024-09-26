mod apps;
mod dmenu;
mod file;
mod run;

pub use apps::AppsMode;
pub use dmenu::DmenuMode;
pub use file::FileMode;
use nucleo_matcher::{
    pattern::{CaseMatching, Normalization, Pattern},
    Config, Matcher,
};
pub use run::RunMode;

use crate::item::Item;

pub trait Mode {
    fn name(&self) -> &str;
    fn options(&mut self) -> Vec<Item>;
    fn run(&mut self, input: &str) -> Vec<Item> {
        let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
        Pattern::parse(input, CaseMatching::Ignore, Normalization::Smart)
            .match_list(self.options(), &mut matcher)
            .into_iter()
            .map(|(item, _)| item)
            .collect()
    }
}

impl dyn Mode {
    pub fn matches(&mut self, input: &str) -> Vec<Item> {
        self.run(input)
            .into_iter()
            .take(100) // Max to display
            .collect()
    }
}
