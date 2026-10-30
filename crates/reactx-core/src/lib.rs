//! Shared terminal-DOM types for ReactX.

pub mod color;
pub mod events;
pub mod node;
pub mod screen;
pub mod static_out;
pub mod style;
pub mod transform;
pub mod wire;

pub use color::{NamedColor, TuiColor};
pub use events::{FocusEvent, KeyModifiers, MouseButtonKind, TuiEvent, TuiKey, TuiKeyEvent};
pub use node::{NodeId, NodeTree, TuiNode};
pub use ratatui::text::{Line, Span};
pub use style::{
    Align, BorderKind, ClassState, Dim, Display, Edges, FlexDir, FlexWrap, Justify, Overflow,
    Position, StyleProps, TextAlign, TextWrap, TitleAlign,
};
