mod apps;
mod dmenu;
mod files;
mod run;

pub use apps::AppsMode;
pub use dmenu::DmenuMode;
pub use files::*;
pub use run::RunMode;

use crate::{item::Item, winit_app::EventHandle};
use nucleo::{
    Config, Matcher,
    pattern::{CaseMatching, Normalization, Pattern},
};

pub trait Mode {
    fn name(&self) -> &str;
    fn run(&mut self, event_handle: EventHandle);
    fn update(&mut self, input: &str) -> Vec<Item>;
}

pub trait SimpleMode {
    fn name(&self) -> &str;
    fn get_items(&mut self) -> &Vec<Item>;
}

impl<T: SimpleMode> Mode for T {
    fn name(&self) -> &str {
        self.name()
    }

    fn run(&mut self, _: EventHandle) {}

    fn update(&mut self, input: &str) -> Vec<Item> {
        fuzzy_match(input, self.get_items())
    }
}

fn fuzzy_match(input: &str, items: &[Item]) -> Vec<Item> {
    let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
    Pattern::parse(input, CaseMatching::Ignore, Normalization::Smart)
        .match_list(items, &mut matcher)
        .into_iter()
        // TODO: avoid cloning the item
        .map(|(item, _)| item.clone())
        .take(64) // Limit the results
        .collect()
}
