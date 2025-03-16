use nucleo::{Config, Nucleo};

use super::Mode;
use crate::{
    file_finder::{self, FileResult},
    item::Item,
    winit_app::EventHandle,
};
use std::{
    cell::RefCell,
    path::PathBuf,
    sync::{mpsc, Arc},
    thread,
    time::Instant,
};

pub struct FilesMode {
    root: PathBuf,
    nucleo: Option<Nucleo<Item>>,
    current_input: String,
}

impl FilesMode {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            nucleo: None,
            current_input: String::new(),
        }
    }
}

impl Mode for FilesMode {
    fn name(&self) -> &str {
        "Files"
    }

    fn run(&mut self, event_handle: EventHandle) {
        let config = Config::DEFAULT.match_paths();

        let event_handle = Arc::new(event_handle);
        thread_local! {
            static LAST_UPDATE: RefCell<Instant> = RefCell::new(Instant::now() - std::time::Duration::from_secs(1));
        }
        let mut nucleo = Nucleo::new(
            config,
            Arc::new(move || {
                LAST_UPDATE.with_borrow_mut(|last_update| {
                    if last_update.elapsed().as_millis() > 10 {
                        *last_update = Instant::now();
                        event_handle.send_update();
                    }
                });
            }),
            None,
            1,
        );
        nucleo.pattern.reparse(
            0,
            "",
            nucleo::pattern::CaseMatching::Ignore,
            nucleo::pattern::Normalization::Smart,
            false,
        );
        nucleo.tick(10);
        let injector = nucleo.injector();
        self.nucleo = Some(nucleo);

        let root = self.root.clone();
        thread::spawn(move || {
            let (files_tx, files_rx) = mpsc::channel::<FileResult>();
            file_finder::find_all_files(&root, files_tx);
            while let Ok(entry) = files_rx.recv() {
                {
                    injector.push(entry.into(), |item, b| {
                        b[0] = item.as_ref().into();
                    });
                }
            }
        });
    }

    fn update(&mut self, input: &str) -> Vec<Item> {
        let nucleo = self.nucleo.as_mut().unwrap();
        let snapshot = nucleo.snapshot();
        let matches = snapshot
            .matched_items(..snapshot.matched_item_count().min(64))
            .map(|item| item.data.clone())
            .collect();
        if input != self.current_input {
            self.current_input = input.to_string();
            nucleo.pattern.reparse(
                0,
                input,
                nucleo::pattern::CaseMatching::Ignore,
                nucleo::pattern::Normalization::Smart,
                false,
            );
        }
        nucleo.tick(10);
        matches
    }
}
