//! ReactX: a React-style terminal UI framework.
//!
//! ```no_run
//! use reactx::prelude::*;
//!
//! #[component]
//! fn Dashboard() -> Element {
//!     let mut count = use_signal(|| 0);
//!     rsx! {
//!         div { style: "direction:column;border:rounded;padding:1",
//!             span { style: "color:#a6e3a1", "Count: {count}" }
//!             Button { on_click: move |_| count += 1, "+1" }
//!         }
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
pub use reactx_tui::events::{handle_event, onmousedrag, Flow};
pub use reactx_tui::{launch, restore_terminal, use_terminal_size, TuiRenderer};
pub use reactx_ui::{
    use_theme, BarChart, Button, Calendar, Canvas, CanvasShape, Chart, ChartDataset, Checkbox,
    Container, Gauge, Input, List, ListItem, Modal, Scrollbar, Sparkline, Table, Tabs, Text,
    TextArea, Theme, ThemeProvider,
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
    pub use reactx_tui::events::{handle_event, onmousedrag, Flow};
    pub use reactx_tui::{launch, restore_terminal, use_terminal_size, TuiRenderer};
    pub use reactx_ui::{
        use_theme, BarChart, Button, Calendar, Canvas, CanvasShape, Chart, ChartDataset, Checkbox,
        Container, Gauge, Input, List, ListItem, Modal, Scrollbar, Sparkline, Table, Tabs, Text,
        TextArea, Theme, ThemeProvider,
    };
}
