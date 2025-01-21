use crate::{
    config::Config,
    mode::{Match, Mode2},
    recent::RecentItems,
    ui::{
        column, container, DynWidget, DynamicList, Editor, Length, TextBuilder, TextEditor, UVec2,
        Widget,
    },
    winit_app::{EventHandle, UserEvent},
};
use cosmic_text::Action;
use winit::{
    event::{ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};

pub struct Launcher {
    mode: Box<dyn Mode2>,
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
    pub fn load(mode: Box<dyn Mode2>) -> anyhow::Result<Self> {
        let config = Config::default();
        let editor = Editor::new();
        // note that the item height must be large enough to fit the text
        let list = DynamicList::new(28, 4);
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

    pub fn start(&mut self, event_handle: EventHandle) {
        self.mode.start(event_handle);
    }

    pub fn root(&self) -> &DynWidget {
        &self.root
    }

    pub fn resize(&mut self, size: UVec2) {
        self.root.layout(size);
    }

    pub fn user_event(&mut self, event: UserEvent) {
        match event {
            UserEvent::Update => {
                self.update();
            }
        }
    }

    pub fn key_input(&mut self, event: KeyEvent) -> bool {
        let mut is_dirty = false;
        if event.state == ElementState::Pressed {
            if event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                self.close_requested = true;
            } else if event.physical_key == PhysicalKey::Code(KeyCode::Enter) {
                is_dirty = true;
                self.close_requested = true;
                if let Err(e) = self.recent.add_and_save(
                    &self.mode.name(),
                    self.matches[self.selected].item().clone(),
                ) {
                    log::error!("Failed to save recent items: {e}");
                }
                // TODO: get exec working
                // self.mode.exec(&self.matches[self.selected].item().clone());
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
        } else if event.state == ElementState::Released {
            if event.physical_key == PhysicalKey::Code(KeyCode::ControlLeft)
                || event.physical_key == PhysicalKey::Code(KeyCode::ControlRight)
            {
                self.ctrl_pressed = false;
            }
        }
        is_dirty
    }

    pub fn update(&mut self) {
        log::info!("handling update");
        let input = self.editor.text();
        // TODO: recents rework
        // if input.is_empty() {
        //     self.matches = self.recent.get_matches(&self.mode.name());
        // } else {
        self.matches = self.mode.update(&input);
        // }
        self.list
            .update(self.matches.iter().enumerate().map(|(i, r#match)| {
                let item_text = format!("{match}");

                container(
                    TextBuilder::new(&item_text)
                        .size(self.config.font_size.normal)
                        .build(),
                )
                .bg(if i == self.selected {
                    self.config.colors.primary
                } else {
                    self.config.colors.background
                })
                .width(Length::Fill)
                .padding((0, 4)) // must fit within the list item height
                .into_dyn()
            }));
    }

    pub fn close_requested(&self) -> bool {
        self.close_requested
    }
}

fn build_ui(mode_name: &str, config: &Config, editor: Editor, list: DynamicList) -> DynWidget {
    let editor = TextEditor::new(editor, config.font_size.normal);
    let root = container(column([
        container(
            TextBuilder::new(mode_name)
                .size(config.font_size.large)
                .bold(true)
                .build(),
        )
        .padding((0, 8))
        .into_dyn(),
        container(
            container(editor)
                .padding((4, 8))
                .bg(config.colors.background_second),
        )
        .padding((0, 8))
        .into_dyn(),
        list.into_dyn(),
    ]))
    .padding_all(32)
    .bg(config.colors.background)
    .width(Length::Fill)
    .height(Length::Fill);
    root.into_dyn()
}
