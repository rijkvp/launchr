use super::SimpleMode;
use crate::item::Item;

pub struct DmenuMode {
    prompt: String,
    options: Vec<Item>,
}

impl DmenuMode {
    pub fn new(prompt: Option<String>, input: String) -> Self {
        let options = input
            .lines()
            .map(|s| Item::new_selection(s.to_string()))
            .collect();
        Self {
            prompt: prompt.unwrap_or("dmenu".to_string()),
            options,
        }
    }
}

impl SimpleMode for DmenuMode {
    fn name(&self) -> &str {
        &self.prompt
    }

    fn get_items(&mut self) -> &Vec<Item> {
        &self.options
    }
}
