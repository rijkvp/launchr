mod apps;
mod dmenu;
mod file;
mod run;

pub use apps::AppsMode;
pub use dmenu::DmenuMode;
pub use file::FileMode;
pub use run::RunMode;

use crate::item::Item;

pub trait Mode {
    fn name(&self) -> &str;
    fn options(&mut self) -> Vec<Item>;
    fn run(&mut self, input: &str) -> Vec<Item> {
        self.options()
            .into_iter()
            .filter(|i| i.text().to_lowercase().contains(&input.to_lowercase()))
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
