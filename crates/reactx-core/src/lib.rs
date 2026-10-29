//! Shared terminal-DOM types for ReactX.

pub mod color;
pub mod events;
pub mod node;
pub mod style;
pub mod wire;

pub use color::{NamedColor, TuiColor};
pub use events::{FocusEvent, KeyModifiers, MouseButtonKind, TuiEvent, TuiKey, TuiKeyEvent};
pub use node::{NodeId, NodeTree, TuiNode};
pub use style::{
    Align, BorderKind, Dim, Display, Edges, FlexDir, Justify, Overflow, Position, StyleProps,
    TextAlign, TitleAlign,
};
