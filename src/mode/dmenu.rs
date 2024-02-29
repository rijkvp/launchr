use super::Mode;
use crate::item::Item;

pub struct DmenuMode {
    options: Vec<Item>,
}

impl DmenuMode {
    pub fn new(input: String) -> Self {
        let options = input
            .lines()
            .map(|s| Item::Selection(s.to_string()))
            .collect();
        Self { options }
    }
}

impl Mode for DmenuMode {
    fn name(&self) -> &str {
        "dmenu"
    }

    fn options(&mut self) -> Vec<Item> {
        self.options.clone()
    }
}
