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
    fn run(&mut self, event_handle: EventHandle);
    fn update(&mut self, input: &str) -> Vec<Item>;
    fn display_name(&self) -> &str;
    fn cache_key(&self) -> Option<&'static str>;
}

pub trait SimpleMode {
    fn display_name(&self) -> &str;
    fn get_items(&mut self) -> &Vec<Item>;
}

impl<T: SimpleMode> Mode for T {
    fn run(&mut self, _: EventHandle) {}

    fn update(&mut self, input: &str) -> Vec<Item> {
        fuzzy_match(input, self.get_items())
    }

    fn display_name(&self) -> &str {
        self.display_name()
    }

    fn cache_key(&self) -> Option<&'static str> {
        None
    }
}

pub fn fuzzy_match(input: &str, items: &[Item]) -> Vec<Item> {
    let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
    Pattern::parse(input, CaseMatching::Ignore, Normalization::Smart)
        .match_list(items, &mut matcher)
        .into_iter()
        // TODO: avoid cloning the item
        .map(|(item, _)| item.clone())
        .take(64) // Limit the results
        .collect()
}
