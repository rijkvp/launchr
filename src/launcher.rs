use crate::{
    config::Config,
    item::Item,
    mode::Mode,
    recent::RecentItems,
    ui::{
        DynWidget, DynamicList, Editor, Length, TextBuilder, TextEditor, UVec2, Widget, column,
        container,
    },
    winit_app::EventHandle,
};
use anyhow::Context;
use cosmic_text::Action;
use winit::{
    event::{ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};

pub struct Launcher {
    mode: Box<dyn Mode>,
    root: DynWidget,
    selected: usize,
    config: Config,
    close_requested: bool,
    ctrl_pressed: bool,
    recent: RecentItems,
    list: DynamicList,
    matches: Vec<Match>,
    editor: Editor,
}

impl Launcher {
    pub fn load(mode: Box<dyn Mode>) -> anyhow::Result<Self> {
        let config = Config::load().context("failed to load config")?;
        let editor = Editor::new(config.font.font_name.clone());
        // NOTE: due to limitations of the layout system, the item height must be large enough to fit the text
        let list = DynamicList::new(32, 4);
        let root = build_ui(mode.name(), &config, editor.clone(), list.clone());
        Ok(Self {
            root,
            mode,
            selected: 0,
            config,
            close_requested: false,
            ctrl_pressed: false,
            recent: RecentItems::load_or_default()?,
            list,
            matches: Vec::new(),
            editor,
        })
    }

    pub fn run(&mut self, event_handle: EventHandle) {
        self.mode.run(event_handle);
    }

    pub fn root(&self) -> &DynWidget {
        &self.root
    }

    pub fn resize(&mut self, size: UVec2) {
        self.root.layout(size);
    }

    pub fn key_input(&mut self, event: KeyEvent) -> bool {
        let mut is_dirty = false;
        if event.state == ElementState::Pressed {
            if event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                self.close_requested = true;
            } else if event.physical_key == PhysicalKey::Code(KeyCode::Enter) {
                is_dirty = true;
                // holding CTRL keeps the launchr open
                if !self.ctrl_pressed {
                    self.close_requested = true;
                }
                if let Err(e) = self
                    .recent
                    .add_and_save(self.mode.name(), self.matches[self.selected].item.clone())
                {
                    log::error!("Failed to save recent items: {e}");
                }
                // Execute the selected match
                self.matches[self.selected].item.exec();
            } else if event.physical_key == PhysicalKey::Code(KeyCode::ArrowDown)
                || self.ctrl_pressed && event.physical_key == PhysicalKey::Code(KeyCode::KeyJ)
            {
                let list_length = self.list.max_items().min(self.matches.len());
                self.selected = (self.selected as i64 + 1).rem_euclid(list_length as i64) as usize;
                log::info!("selected: {}", self.selected);
                is_dirty = true;
            } else if event.physical_key == PhysicalKey::Code(KeyCode::ArrowUp)
                || self.ctrl_pressed && event.physical_key == PhysicalKey::Code(KeyCode::KeyK)
            {
                let list_length = self.list.max_items().min(self.matches.len());
                self.selected = (self.selected as i64 - 1).rem_euclid(list_length as i64) as usize;
                log::info!("selected: {}", self.selected);
                is_dirty = true;
            } else if event.physical_key == PhysicalKey::Code(KeyCode::ControlLeft)
                || event.physical_key == PhysicalKey::Code(KeyCode::ControlRight)
            {
                self.ctrl_pressed = true;
            } else if self.ctrl_pressed && event.physical_key == PhysicalKey::Code(KeyCode::KeyC) {
                self.close_requested = true;
            } else {
                // Editor input
                if let PhysicalKey::Code(key) = event.physical_key {
                    is_dirty = self.editor.handle_key(key);
                }
                if let Some(char) = event.text.and_then(|t| t.chars().next()) {
                    self.editor.perform_action(Action::Insert(char));
                    self.selected = 0;
                    is_dirty = true;
                }
            }
        } else if event.state == ElementState::Released
            && (event.physical_key == PhysicalKey::Code(KeyCode::ControlLeft)
                || event.physical_key == PhysicalKey::Code(KeyCode::ControlRight))
        {
            self.ctrl_pressed = false;
        }
        is_dirty
    }

    pub fn update(&mut self) {
        log::info!("[UPDATE METHOD]");
        let input = self.editor.text();
        self.matches.clear();
        if input.is_empty() {
            self.matches
                .extend(
                    self.recent
                        .get_matches(self.mode.name())
                        .into_iter()
                        .map(|item| Match {
                            item,
                            is_recent: true,
                        }),
                );
        }
        self.matches
            .extend(self.mode.update(&input).into_iter().map(|item| Match {
                item,
                is_recent: false,
            }));

        self.list
            .update(self.matches.iter().enumerate().map(|(i, r#match)| {
                let mut item_text = format!("{}", r#match.item);
                if r#match.is_recent {
                    item_text.push_str(" (recent)");
                }

                container(
                    TextBuilder::new(&item_text)
                        .size(self.config.font.normal_size)
                        .font(self.config.font.font_name.as_ref())
                        .build(),
                )
                .bg(if i == self.selected {
                    self.config.color.primary
                } else {
                    self.config.color.background
                })
                .width(Length::Fill)
                .padding((4, 8)) // must fit within the list item height
                .into_dyn()
            }));
    }

    pub fn close_requested(&self) -> bool {
        self.close_requested
    }
}

struct Match {
    item: Item,
    is_recent: bool,
}

fn build_ui(mode_name: &str, config: &Config, editor: Editor, list: DynamicList) -> DynWidget {
    let editor = TextEditor::new(editor, config.font.normal_size);
    let root = container(column([
        container(
            TextBuilder::new(mode_name)
                .size(config.font.large_size)
                .font(config.font.font_name.as_ref())
                .bold(true)
                .build(),
        )
        .padding((0, 4))
        .into_dyn(),
        container(
            container(editor)
                .padding((4, 8))
                .bg(config.color.background_second),
        )
        .padding((0, 8))
        .into_dyn(),
        list.into_dyn(),
    ]))
    .padding_all(32)
    .bg(config.color.background)
    .width(Length::Fill)
    .height(Length::Fill);
    root.into_dyn()
}
