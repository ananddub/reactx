use ratatui::backend::TestBackend;
use ratatui::layout::Rect;
use ratatui::Terminal;
use reactx::prelude::*;

pub(crate) struct Harness {
    dom: VirtualDom,
    pub(crate) renderer: TuiRenderer,
    terminal: Terminal<TestBackend>,
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

    pub(crate) fn attrs(&self, id: NodeId) -> &StyleProps {
        &self.renderer.tree.get(id).expect("node exists").attrs
    }

    pub(crate) fn rect(&self, id: NodeId) -> Rect {
        self.renderer.layout[&id]
    }

    pub(crate) fn click_text(&mut self, tag: &str, text: &str) {
        let node = self.node_where(|node| {
            node.tag == tag && self.renderer.tree.collect_text(node.id).contains(text)
        });
        let rect = self.rect(node);
        self.event(TuiEvent::MouseClick {
            x: rect.x + rect.width / 2,
            y: rect.y + rect.height / 2,
            modifiers: KeyModifiers::NONE,
        });
    }

    pub(crate) fn mouse_move(&mut self, x: u16, y: u16) {
        self.event(TuiEvent::MouseMove {
            x,
            y,
            modifiers: KeyModifiers::NONE,
        });
    }

    pub(crate) fn resize(&mut self, width: u16, height: u16) {
        self.terminal.backend_mut().resize(width, height);
        self.settle();
    }
}
