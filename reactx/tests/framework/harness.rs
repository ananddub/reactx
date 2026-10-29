use ratatui::backend::TestBackend;
use ratatui::Terminal;
use reactx::prelude::*;

pub(crate) struct Harness {
    pub(crate) dom: VirtualDom,
    pub(crate) renderer: TuiRenderer,
    pub(crate) terminal: Terminal<TestBackend>,
}

impl Harness {
    pub(crate) fn new(app: fn() -> Element) -> Self {
        let mut dom = VirtualDom::new(app);
        let mut renderer = TuiRenderer::new();
        dom.rebuild(&mut renderer);
        let mut terminal = Terminal::new(TestBackend::new(60, 16)).unwrap();
        renderer.paint(&mut terminal).unwrap();
        Self {
            dom,
            renderer,
            terminal,
        }
    }

    pub(crate) fn screen(&self) -> String {
        let buffer = self.terminal.backend().buffer();
        let mut out = String::new();
        for y in 0..buffer.area.height {
            for x in 0..buffer.area.width {
                out.push_str(buffer[(x, y)].symbol());
            }
            out.push('\n');
        }
        out
    }

    pub(crate) fn settle(&mut self) {
        self.dom.render_immediate(&mut self.renderer);
        self.renderer.paint(&mut self.terminal).unwrap();
    }

    pub(crate) fn event(&mut self, event: TuiEvent) {
        assert_eq!(
            handle_event(&self.dom, &mut self.renderer, event),
            Flow::Continue
        );
        self.settle();
    }

    pub(crate) fn node_where(&self, mut predicate: impl FnMut(&TuiNode) -> bool) -> NodeId {
        self.renderer
            .tree
            .nodes_in_order()
            .into_iter()
            .find(|id| predicate(self.renderer.tree.get(*id).unwrap()))
            .unwrap_or_else(|| panic!("no node matched; screen:\n{}", self.screen()))
    }

    pub(crate) fn node_text(&self, id: NodeId) -> String {
        self.renderer.tree.collect_text(id)
    }

    pub(crate) fn button(&self, text: &str) -> NodeId {
        self.node_where(|node| node.tag == "button" && self.node_text(node.id) == text)
    }

    pub(crate) fn value(&self, id: NodeId) -> String {
        self.renderer
            .tree
            .get(id)
            .and_then(|node| node.attrs.value.clone())
            .unwrap_or_default()
    }

    pub(crate) fn click(&mut self, x: u16, y: u16) {
        self.event(TuiEvent::MouseClick { x, y });
    }

    pub(crate) fn wheel(&mut self, x: u16, y: u16, delta_x: i32, delta_y: i32) {
        self.event(TuiEvent::Scroll {
            x,
            y,
            delta_x,
            delta_y,
        });
    }

    pub(crate) fn key(&mut self, key: TuiKey) {
        self.key_mod(key, KeyModifiers::NONE);
    }

    pub(crate) fn key_mod(&mut self, key: TuiKey, modifiers: KeyModifiers) {
        self.event(TuiEvent::Key(TuiKeyEvent { key, modifiers }));
    }

    pub(crate) fn ctrl(&mut self, ch: char) {
        self.key_mod(
            TuiKey::Char(ch),
            KeyModifiers {
                ctrl: true,
                ..KeyModifiers::NONE
            },
        );
    }

    pub(crate) fn shift(&mut self, key: TuiKey) {
        self.key_mod(
            key,
            KeyModifiers {
                shift: true,
                ..KeyModifiers::NONE
            },
        );
    }

    pub(crate) fn type_text(&mut self, text: &str) {
        for ch in text.chars() {
            self.key(TuiKey::Char(ch));
        }
    }

    pub(crate) fn mouse_down(&mut self, x: u16, y: u16, button: MouseButtonKind) {
        self.event(TuiEvent::MouseDown { x, y, button });
    }

    pub(crate) fn mouse_up(&mut self, x: u16, y: u16, button: MouseButtonKind) {
        self.event(TuiEvent::MouseUp { x, y, button });
    }

    pub(crate) fn mouse_move(&mut self, x: u16, y: u16) {
        self.event(TuiEvent::MouseMove { x, y });
    }

    pub(crate) fn mouse_drag(&mut self, x: u16, y: u16, button: MouseButtonKind) {
        self.event(TuiEvent::MouseDrag { x, y, button });
    }
}
