//! ReactX: a React-style terminal UI framework.
//!
//! ```no_run
//! use reactx::prelude::*;
//!
//! fn Dashboard() -> Element {
//!     let mut count = use_signal(|| 0);
//!     jsx! {
//!         <ThemeProvider>
//!             <Container class="flex flex-col border border-blue-300 rounded p-1 gap-1">
//!                 <span class="text-green-300 bold">Count: {count}</span>
//!                 <Button on_click=move |_| count += 1>+1</Button>
//!             </Container>
//!         </ThemeProvider>
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     reactx::launch(Dashboard).await;
//! }
//! ```

pub mod jsx;
pub mod store;

pub use dioxus::prelude::*;
pub use jsx::{jsx_node, IntoJsxElement, IntoJsxNode};
pub use reactx_async::{use_fetch, use_websocket, SocketState, Suspense};
pub use reactx_core::{
    Align, BorderKind, Dim, Edges, FlexDir, Justify, KeyModifiers, MouseButtonKind, NamedColor,
    NodeId, NodeTree, Overflow, Position, StyleProps, TextAlign, TitleAlign, TuiColor, TuiEvent,
    TuiKey, TuiKeyEvent, TuiNode,
};
pub use reactx_macro::jsx;
pub use reactx_router::{
    match_route, navigate, routes, use_location, use_param, use_query, use_router, Link, Query,
    Route, RouteMatch, Router, RouterContext,
};
pub use reactx_tui::effects::{
    fx, register_shader, screen_effects, tachyonfx, Color, Effect, Interpolation, Motion,
    ScreenEffects,
};
pub use reactx_tui::events::{handle_event, onmousedrag, Flow, TuiClipboardEvent};
pub use reactx_tui::{
    launch, restore_terminal, use_animation_frame, use_color_tween, use_spring, use_terminal_focus,
    use_terminal_size, use_ticker, use_tween, Spring, TuiRenderer, Tween,
};
pub use reactx_ui::{
    use_theme, Animate, BarChart, Button, Calendar, Canvas, CanvasShape, Chart, ChartDataset,
    Checkbox, Container, Gauge, Input, List, ListItem, Modal, Scrollbar, Sparkline, Table, Tabs,
    Text, TextArea, Theme, ThemeProvider,
};
pub use store::Store;

/// Everything needed to build an app.
pub mod prelude {
    pub use dioxus::prelude::*;

    pub use crate::jsx::{jsx_node, IntoJsxElement, IntoJsxNode};
    pub use crate::store::Store;
    pub use reactx_async::{use_fetch, use_websocket, SocketState, Suspense};
    pub use reactx_core::{
        BorderKind, Dim, Edges, KeyModifiers, MouseButtonKind, NamedColor, NodeId, NodeTree,
        StyleProps, TuiColor, TuiEvent, TuiKey, TuiKeyEvent, TuiNode,
    };
    pub use reactx_macro::jsx;
    pub use reactx_router::{
        navigate, routes, use_location, use_param, use_query, use_router, Link, Query, Route,
        RouteMatch, Router, RouterContext,
    };
    pub use reactx_tui::effects::{
        fx, register_shader, screen_effects, tachyonfx, Color, Effect, Interpolation, Motion,
        ScreenEffects,
    };
    pub use reactx_tui::events::{handle_event, onmousedrag, Flow, TuiClipboardEvent};
    pub use reactx_tui::{
        launch, restore_terminal, use_animation_frame, use_color_tween, use_spring,
        use_terminal_focus, use_terminal_size, use_ticker, use_tween, Spring, TuiRenderer, Tween,
    };
    pub use reactx_ui::{
        use_theme, Animate, BarChart, Button, Calendar, Canvas, CanvasShape, Chart, ChartDataset,
        Checkbox, Container, Gauge, Input, List, ListItem, Modal, Scrollbar, Sparkline, Table,
        Tabs, Text, TextArea, Theme, ThemeProvider,
    };
}
