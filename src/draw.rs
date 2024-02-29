
fn render_matches(&self, outer: Rect) -> Vec<Component> {
    let mut components = Vec::new();
    for (i, item) in self.matches.iter().enumerate() {
        let color = if i == self.selected {
            Color::from_rgba8(200, 200, 200, 50)
        } else {
            Color::from_rgba8(0, 0, 255, 50)
        };
        let y = outer.y + i as u64 * FONT_SIZE;
        if y > outer.height {
            break;
        }
        components.push(Component::Container(Container::new(
            Rect::new(outer.x, y, outer.width, FONT_SIZE),
            color,
        )));
        components.push(Component::Text(
            Text::new(
                Rect::new(outer.x, y, outer.width, FONT_SIZE),
                FONT_SIZE as f32,
            )
            .with_text(&item.display()),
        ));
    }
    components
}
