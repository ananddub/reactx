use ratatui::backend::TestBackend;
use ratatui::Terminal;
use reactx::prelude::*;

pub(crate) struct Harness {
    dom: VirtualDom,
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

    fn settle(&mut self) {
        self.dom.render_immediate(&mut self.renderer);
        self.renderer.paint(&mut self.terminal).unwrap();
    }

    pub(crate) fn node_where(&self, mut predicate: impl FnMut(&TuiNode) -> bool) -> NodeId {
        self.renderer
            .tree
            .nodes_in_order()
            .into_iter()
            .find(|id| predicate(self.renderer.tree.get(*id).unwrap()))
            .unwrap_or_else(|| panic!("no node matched; screen:\n{}", self.screen()))
    }

    pub(crate) fn click_text(&mut self, tag: &str, text: &str) {
        let node = self.node_where(|node| {
            node.tag == tag && self.renderer.tree.collect_text(node.id).contains(text)
        });
        let rect = self.renderer.layout[&node];
        let event = TuiEvent::MouseClick {
            x: rect.x + rect.width / 2,
            y: rect.y + rect.height / 2,
        };
        assert_eq!(
            handle_event(&self.dom, &mut self.renderer, event),
            Flow::Continue
        );
        self.settle();
    }

    fn key(&mut self, key: TuiKey) {
        let event = TuiEvent::Key(TuiKeyEvent {
            key,
            modifiers: KeyModifiers::NONE,
        });
        assert_eq!(
            handle_event(&self.dom, &mut self.renderer, event),
            Flow::Continue
        );
        self.settle();
    }

    pub(crate) fn type_text(&mut self, text: &str) {
        for ch in text.chars() {
            self.key(TuiKey::Char(ch));
        }
    }
}
