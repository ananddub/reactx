//! Registry for user-defined node shaders.

use std::cell::RefCell;
use std::rc::Rc;

use ratatui::layout::Rect;
use rustc_hash::FxHashMap;
use tachyonfx::Effect;

type Factory = Rc<dyn Fn(Rect) -> Effect>;

thread_local! {
    static SHADERS: RefCell<FxHashMap<String, Factory>> = RefCell::new(FxHashMap::default());
}

/// Register a named effect that `animate="name"` and `<Animate effect="name">`
/// can use. The factory receives the visible rectangle of the animated node;
/// it only needs to build a [`tachyonfx`] effect — the area is applied for it.
///
/// ```ignore
/// register_shader("sparkle", |_| fx::dissolve((400, Interpolation::SineIn)));
/// ```
///
/// Registration is per-thread, so call it on the thread that runs
/// [`launch`](crate::launch) (usually `main`, before launching).
pub fn register_shader(name: impl Into<String>, factory: impl Fn(Rect) -> Effect + 'static) {
    let name = name.into().trim().to_ascii_lowercase();
    if name.is_empty() {
        return;
    }
    SHADERS.with(|shaders| {
        shaders.borrow_mut().insert(name, Rc::new(factory));
    });
}

pub(crate) fn custom_effect(name: &str, area: Rect) -> Option<Effect> {
    SHADERS.with(|shaders| shaders.borrow().get(name).map(|factory| factory(area)))
}
