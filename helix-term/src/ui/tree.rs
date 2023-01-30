use crate::{compositor::{Component, EventResult, self, Compositor}, ctrl, key};
use helix_view::{graphics::Margin, input::{Event, self}};
use tui::{
    buffer::Buffer as Surface,
    widgets::{Block, Borders, Widget},
};

pub struct FileTree {}

impl FileTree {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for FileTree {
    fn handle_event(&mut self, event: &input::Event, _ctx: &mut compositor::Context) -> crate::compositor::EventResult {
        let key_event =  match event {
            Event::Key(event) => *event,
            Event::Resize(..) => return EventResult::Consumed(None),
            _ => return EventResult::Ignored(None),
        };

        let close_fn = EventResult::Consumed(Some(Box::new(|compositor: &mut Compositor, _cx| {
            compositor.last_picker = compositor.pop();
        })));

        match key_event {
            key!(Esc) | key!('q') | ctrl!('c') => {
                return close_fn;
            }
            _ => EventResult::Consumed(None)
        }
    }

    fn render(
        &mut self,
        area: helix_view::graphics::Rect,
        surface: &mut Surface,
        cx: &mut compositor::Context,
    ) {
        // -- Render the frame:
        // clear area
        let background = cx.editor.theme.get("ui.background");
        let text = cx.editor.theme.get("ui.text");
        surface.clear_with(area, background);

        let block = Block::default().borders(Borders::ALL);
        let inner = block.inner(area);
        let margin = Margin::horizontal(1);
        let inner = inner.inner(&margin);
        block.render(area, surface);

        let content = "hello, world; this is the beginning of the file tree";
        let x = inner.x + inner.width.saturating_sub(content.len() as u16) / 2;
        let y = inner.y + inner.height / 2;
        surface.set_stringn(x, y, content, inner.width as usize, text);
    }
}
