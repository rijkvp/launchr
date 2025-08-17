use crate::item::Item;
use bincode::{Decode, Encode};
use std::{
    collections::BTreeMap,
    fs::{self, File},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

#[derive(Encode, Decode)]
struct RecentItem {
    item: Item,
    time: u64,
}

impl RecentItem {
    fn new(item: Item) -> Self {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self { item, time }
    }
}

#[derive(Default, Encode, Decode)]
pub struct RecentItems {
    items: BTreeMap<String, Vec<RecentItem>>,
}

const STATE_DIR_NAME: &str = env!("CARGO_CRATE_NAME");
const RECENT_FILE_NAME: &str = "recent";
const MAX_RECENT_ITEMS: usize = 12;

impl RecentItems {
    pub fn load_or_default() -> anyhow::Result<Self> {
        let start_instant = Instant::now();
        let state_dir = dirs::state_dir().unwrap().join(STATE_DIR_NAME);
        fs::create_dir_all(&state_dir)?;
        let path = state_dir.join(RECENT_FILE_NAME);
        if !path.exists() {
            return Ok(Self::default());
        }
        let mut file = File::open(path)?;
        let res: Self = bincode::decode_from_std_read(&mut file, bincode::config::standard())?;
        log::info!("loaded recent items in {:?}", start_instant.elapsed());
        Ok(res)
    }

    pub fn add_and_save(&mut self, mode: &str, item: Item) -> anyhow::Result<()> {
        let mode_items = self.items.entry(mode.to_string()).or_default();
        if let Some(index) = mode_items.iter().position(|r| r.item == item) {
            mode_items.remove(index);
        }
        mode_items.push(RecentItem::new(item));
        mode_items.drain(..mode_items.len().saturating_sub(MAX_RECENT_ITEMS));
        self.save()
    }

    fn save(&self) -> anyhow::Result<()> {
        let state_dir = dirs::state_dir().unwrap().join(STATE_DIR_NAME);
        fs::create_dir_all(&state_dir)?;
        let file = state_dir.join(RECENT_FILE_NAME);
        let mut file = File::create(file)?;
        bincode::encode_into_std_write(self, &mut file, bincode::config::standard())?;
        Ok(())
    }

    pub fn get_matches(&self, mode: &str) -> Vec<Item> {
        if let Some(items) = self.items.get(mode) {
            let mut matches: Vec<Item> = items.iter().map(|r| r.item.clone()).collect();
            matches.reverse();
            matches
        } else {
            Vec::new()
        }
    }
}
