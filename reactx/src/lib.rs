//! ReactX: a React-style terminal UI framework.
//!
//! ```no_run
//! use reactx::prelude::*;
//!
//! fn Dashboard() -> Element {
//!     let mut count = use_signal(|| 0);
//!     jsx! {
//!         <ThemeProvider>
//!             <Box class="flex flex-col border border-blue-300 rounded p-1 gap-1">
//!                 <span class="text-green-300 bold">Count: {count}</span>
//!                 <Button on_click=move |_| count += 1>+1</Button>
//!             </Box>
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
#[cfg(feature = "async")]
pub use reactx_async::{use_fetch, use_websocket, SocketState, Suspense};
pub use reactx_core::{
    Align, BorderKind, Dim, Edges, FlexDir, Justify, KeyModifiers, Line, MouseButtonKind,
    NamedColor, NodeId, NodeTree, Overflow, Position, Span, StyleProps, TextAlign, TextWrap,
    TitleAlign, TuiColor, TuiEvent, TuiKey, TuiKeyEvent, TuiNode,
};
pub use reactx_macro::jsx;
#[cfg(feature = "router")]
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
    install_panic_hook, launch, launch_inline, launch_inline_with, launch_with, measure_element,
    render_to_string, restore_terminal, use_animation_frame, use_app, use_box_metrics,
    use_color_tween, use_cursor, use_focus, use_focus_manager, use_input, use_input_when,
    use_is_screen_reader_enabled, use_paste, use_paste_when, use_spring, use_stderr, use_stdin,
    use_stdout, use_terminal_focus, use_terminal_size, use_ticker, use_tween, use_window_size,
    AppHandle, BoxMetrics, CursorHandle, FocusManagerHandle, LaunchOptions, RenderCallback,
    RenderMetrics, Spring, StderrHandle, StdinHandle, StdoutHandle, TuiRenderer, Tween,
};
pub use reactx_ui::{
    use_theme, Animate, BarChart, Box, Button, Calendar, Canvas, CanvasShape, Chart, ChartDataset,
    Checkbox, Gauge, Input, List, ListItem, Modal, Newline, Scrollbar, Spacer, Sparkline, Static,
    Table, Tabs, Text, TextArea, Theme, ThemeProvider, Transform,
};
pub use store::Store;

/// Everything needed to build an app.
pub mod prelude {
    pub use dioxus::prelude::*;

    pub use crate::jsx::{jsx_node, IntoJsxElement, IntoJsxNode};
    pub use crate::store::Store;
    #[cfg(feature = "async")]
    pub use reactx_async::{use_fetch, use_websocket, SocketState, Suspense};
    pub use reactx_core::{
        BorderKind, Dim, Edges, KeyModifiers, Line, MouseButtonKind, NamedColor, NodeId, NodeTree,
        Span, StyleProps, TuiColor, TuiEvent, TuiKey, TuiKeyEvent, TuiNode,
    };
    pub use reactx_macro::jsx;
    #[cfg(feature = "router")]
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
        install_panic_hook, launch, launch_inline, launch_inline_with, launch_with,
        measure_element, render_to_string, restore_terminal, use_animation_frame, use_app,
        use_box_metrics, use_color_tween, use_cursor, use_focus, use_focus_manager, use_input,
        use_input_when, use_is_screen_reader_enabled, use_paste, use_paste_when, use_spring,
        use_stderr, use_stdin, use_stdout, use_terminal_focus, use_terminal_size, use_ticker,
        use_tween, use_window_size, AppHandle, BoxMetrics, CursorHandle, FocusManagerHandle,
        LaunchOptions, RenderCallback, RenderMetrics, Spring, StderrHandle, StdinHandle,
        StdoutHandle, TuiRenderer, Tween,
    };
    pub use reactx_ui::{
        use_theme, Animate, BarChart, Box, Button, Calendar, Canvas, CanvasShape, Chart,
        ChartDataset, Checkbox, Gauge, Input, List, ListItem, Modal, Newline, Scrollbar, Spacer,
        Sparkline, Static, Table, Tabs, Text, TextArea, Theme, ThemeProvider, Transform,
    };
}
