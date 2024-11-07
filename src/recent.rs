use crate::{item::Item, mode::Match};
use rkyv::{
    api::high::to_bytes_in, rancor::Error, ser::writer::IoWriter, Archive, Deserialize, Serialize,
};
use std::{
    collections::BTreeMap,
    fs::{self, File},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

#[derive(Archive, Deserialize, Serialize)]
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

#[derive(Default, Archive, Deserialize, Serialize)]
pub struct RecentItems {
    items: BTreeMap<String, Vec<RecentItem>>,
}

impl RecentItems {
    pub fn load_or_default() -> anyhow::Result<Self> {
        let start_instant = Instant::now();
        let state_dir = dirs::state_dir().unwrap().join("launcher");
        fs::create_dir_all(&state_dir)?;
        let path = state_dir.join("recent");
        if !path.exists() {
            return Ok(Self::default());
        }
        let bytes = fs::read(path)?;
        let deserialized = rkyv::from_bytes::<_, Error>(&bytes)?;
        log::info!("loaded recent items in {:?}", start_instant.elapsed());
        Ok(deserialized)
    }

    pub fn add_and_save(&mut self, mode: &str, item: Item) -> anyhow::Result<()> {
        let mode_items = self.items.entry(mode.to_string()).or_default();
        if let Some(index) = mode_items.iter().position(|r| r.item == item) {
            mode_items.remove(index);
        }
        mode_items.push(RecentItem::new(item));
        self.save()
    }

    fn save(&self) -> anyhow::Result<()> {
        let state_dir = dirs::state_dir().unwrap().join("launcher");
        fs::create_dir_all(&state_dir)?;
        let file = state_dir.join("recent");
        let file = File::create(file)?;
        let mut io_writer = IoWriter::new(file);
        to_bytes_in::<_, Error>(self, &mut io_writer)?;
        Ok(())
    }

    pub fn get_matches(&self, mode: &str) -> Vec<Match> {
        if let Some(items) = self.items.get(mode) {
            let mut matches: Vec<Match> = items
                .iter()
                .map(|r| Match::new(r.item.clone(), 0))
                .collect();
            matches.reverse();
            matches
        } else {
            Vec::new()
        }
    }
}
