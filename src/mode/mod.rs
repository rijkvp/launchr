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
}

impl dyn Mode {
    pub fn matches(&mut self, input: &str) -> Vec<Item> {
        self.options()
            .into_iter()
            .filter(|i| i.text().contains(input))
            .take(100) // Max to display
            .collect()
    }
}
