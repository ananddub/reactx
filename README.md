# ReactX

A React-style terminal UI framework for Rust. Components, hooks and RSX come
from [Dioxus](https://dioxuslabs.com) 0.6; layout is Flexbox via
[Taffy](https://github.com/DioxusLabs/taffy); drawing and input run through
[Ratatui](https://ratatui.rs) and [Crossterm](https://github.com/crossterm-rs/crossterm);
async work runs on [Tokio](https://tokio.rs).

```rust
use reactx::prelude::*;

#[component]
fn Dashboard() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        Container { border: "rounded", direction: "column", padding: "1",
            Text { color: "#a6e3a1", "Count: {count}" }
            Button { on_click: move |_| count += 1, "+1" }
        }
    }
}

#[tokio::main]
async fn main() {
    reactx::launch(Dashboard).await;
}
```

> **Why `Container` instead of `Box`?** The `rsx!` macro expands to code that
> refers to `Box<[Attribute]>` unqualified, so naming a component `Box` breaks
> every surrounding `rsx!` invocation. The layout component is therefore called
> `Container` (attributes `direction`, `justify`, `align`, `gap`, `padding`,
> `border`, ... are unchanged).

## JSX syntax

`jsx!` accepts genuine HTML-like markup and compiles it to `rsx!` at build time,
so components, hooks and the renderer stay exactly the same:

```rust
use reactx::prelude::*;

fn App() -> Element {
    let mut count = use_signal(|| 0);
    jsx! {
        <Container border="rounded" border-color="#89b4fa" padding="1">
            <Text color="#a6e3a1" bold>Count: {count}</Text>
            <Button on_click=move |_| count += 1>+1</Button>
        </Container>
    }
}
```

- `attr="text"`, `attr={expression}`, `on_click=move |_| ...` and boolean
  shorthand (`bold`, `disabled`) are supported.
- Hyphenated names become underscores (`border-color` → `border_color`).
- `{expression}` accepts anything implementing `IntoJsxNode`: strings, numbers,
  signals, elements, `Option`s and iterator maps. Conditionals and `match`
  expressions work too:
  ```rust
  {if done { jsx! { <Text dim>done</Text> } } else { jsx! { <Text>todo</Text> } }}
  {items.iter().map(|item| jsx! { <Text key="{item.id}">{item.label}</Text> })}
  ```
- Fragments (`<>...</>`) and comments (`<!-- ... -->`) work; comments are
  dropped.
- Text punctuation is repaired (`Count: {n}` keeps the colon attached, `50 %`
  becomes `50%`, `Ctrl + C` becomes `Ctrl+C`, `open-meteo` keeps its hyphen).

## Workspace

| Crate | Responsibility |
| --- | --- |
| `reactx-core` | `NodeTree`/`TuiNode` DOM, style props, colours, terminal events |
| `reactx-tui` | Dioxus mutation bridge, Taffy layout, Ratatui painter, event loop |
| `reactx-ui` | Themed widgets (`Container`, `Text`, `Button`, `Input`, `TextArea`, `Checkbox`, `Tabs`, `List`, `Modal`), data widgets (`Gauge`, `Table`, `BarChart`, `Sparkline`, `Chart`, `Canvas`, `Scrollbar`, `Calendar`) and `ThemeProvider`/`use_theme` |
| `reactx-async` | `use_fetch`, `use_websocket`, `<Suspense>` |
| `reactx-router` | `Router`, `routes!`, `Link`, `use_location`/`use_param`/`use_query`, in-memory history |
| `reactx-macro` | `jsx!`: compiles HTML-like tags into `rsx!` with [rstml](https://github.com/rs-tml/rstml) |
| `reactx` | Facade and `reactx::prelude::*`, plus the global `Store<T>` |

## Styling

Styles are ordinary attributes; the renderer maps them onto Taffy and Ratatui.
`style: "key:value;..."` is also accepted and uses the same names.

- **Layout**: `direction` (`row`/`column`/`*-reverse`), `justify`, `align`,
  `align_self`, `flex_grow`, `flex_shrink`, `width`, `height`, `min_*`, `max_*`,
  `padding`, `margin`, `gap`, `position`, `top`/`right`/`bottom`/`left`,
  `display`, `overflow`, `scroll_x`, `scroll_y`. Dimensions accept cells
  (`"10"`), percentages (`"50%"`) or `"auto"`.
- **Visual**: `border` (`single`/`rounded`/`double`/`thick`), `border_color`,
  `bg`, `color`, `title`, `title_align`, `text_align`, `layer` (`z_index`).
- **Text**: `bold`, `italic`, `underline`, `strikethrough`, `dim`, `reversed`.
- **Widget state**: `focusable`, `disabled`, `value`, `placeholder`, `label`,
  `checked`, `variant`.
- **Overlays**: `focus_trap` restricts the focus ring and pointer events to a
  subtree, `autofocus` picks the preferred target inside it, and `layer` orders
  siblings (`layer:200` paints above `layer:100`).

Colours accept names (`red`, `bright-green`, ...), `#RGB`, `#RRGGBB`,
`rgb(r, g, b)` and `ansi(n)`.

## Data widgets

Gauges, tables, charts, canvas, scrollbars and the calendar share the same
element tree: each component encodes its props into custom attributes, Taffy
reserves a rectangle (sensible default heights included), and the painter calls
the matching Ratatui widget.

```rust
jsx! {
    <Gauge value={0.82} label="82% Synced" border="rounded" title="Progress" />
    <Gauge value={45.0} use_line={true} />

    <Table
        headers={vec!["PID".into(), "Process".into(), "CPU".into()]}
        rows={rows}
        widths={Some(vec![6, 18, 8])}
        selected_row={Some(selected())}
        title="Processes"
    />

    <BarChart data={vec![("Rust".into(), 95), ("Go".into(), 80)]} horizontal={true} bar_width={Some(1)} />
    <Sparkline data={vec![10, 25, 40, 20, 60, 95, 70]} color="#f9e2af" />

    <Chart
        datasets={vec![ChartDataset {
            name: "Server A".into(),
            data: vec![(0.0, 15.0), (1.0, 22.0), (2.0, 18.0)],
            color: Some("#a6e3a1".into()),
            marker: "braille".into(),
        }]}
        x_bounds={(0.0, 3.0)}
        y_bounds={(0.0, 50.0)}
    />

    <Canvas
        x_bounds={(0.0, 80.0)}
        y_bounds={(0.0, 20.0)}
        marker="block" // braille (default), dot, block or halfblock
        shapes={vec![
            CanvasShape::Line { x1: 6.0, y1: 4.0, x2: 74.0, y2: 16.0, color: None },
            CanvasShape::Rect { x: 30.0, y: 5.0, width: 18.0, height: 10.0, color: Some("#fab387".into()) },
            CanvasShape::Circle { x: 6.0, y: 4.0, radius: 4.0, color: Some("#f38ba8".into()) },
            CanvasShape::Points { coords: vec![(6.0, 4.0), (74.0, 16.0)], color: Some("#89b4fa".into()) },
            CanvasShape::Print { x: 10.0, y: 4.5, text: "eu-1".into(), color: None },
            // CanvasShape::WorldMap { color: None } also renders the world map
        ]}
        title="Global nodes"
        border="single"
    />

    <Scrollbar position={scroll()} content_length={100} orientation="vertical-right" />

    <Calendar year={2026} month={10} selected_day={Some(day())}
        on_select={move |value| day.set(value)} />
}
```

`Calendar` is a regular Dioxus tree of focusable cells: days react to clicks and
`Enter`/`Space`, and `on_month_change` drives the `<`/`>` header buttons. In
horizontal bar charts `bar_width` is the bar thickness in rows. Run the
showcase with `cargo run -p reactx --example dashboard`; it reads
`use_terminal_size()` and shrinks the top row on short terminals.

`Canvas` uses braille dots by default; `marker="block"`, `"dot"` or
`"halfblock"` switches the glyph grid when a terminal renders braille poorly.

Canvas shapes are drawn in the virtual space given by `x_bounds`/`y_bounds`, so
pick bounds roughly proportional to the panel (a 4:1 or 2:1 space for a wide,
short panel). A `Circle` with `radius: 4.0` inside `(-180.0, 180.0)` is less than
one cell wide and therefore invisible; size the radius relative to the bounds.

## Scroll containers

Scrolling is a style value, not a sliced list: put `overflow:hidden` and a
`scroll-y`/`scroll-x` offset on a box and the renderer shifts its descendants,
clips them to the border/padding box, and keeps hit testing in sync.

```rust
let mut offset = use_signal(|| 0u16);
rsx! {
    div {
        style: "height:10;overflow:hidden;scroll-y:{offset}",
        onwheel: move |event: Event<WheelData>| {
            let dy = event.delta().strip_units().y as i32;
            offset.set((offset() as i32 + dy).clamp(0, max) as u16);
        },
        for row in rows {
            span { key: "{row.id}", "{row.label}" }
        }
    }
}
```

Data widgets are cropped the same way: a scrolled `Table` or `Chart` keeps its
row geometry instead of restarting from the first row. Wheel events arrive as
`TuiEvent::Scroll`, bubble to the nearest `onwheel` handler and carry line
deltas (`delta_x`, `delta_y`).

## Interaction

- `Tab`/`Shift+Tab` move focus; the first focusable node is focused on the first
  frame. Focused borders use the focus colour.
- `Enter` or `Space` clicks buttons, checkboxes and any node with an `onclick`.
- Typing edits the focused `Input` or `TextArea`. `Left`/`Right`/`Home`/`End`
  move the caret; `Up`/`Down` move between lines in a textarea. `Shift` with
  those keys extends the selection, `Ctrl+A` selects everything,
  `Ctrl+C`/`Ctrl+X` copy or cut, and `Backspace`/`Delete` replace the
  selection. Pasting inserts at the caret (newlines are kept only in a
  textarea). `oninput` fires with the new value when it changes, and
  `onchange` fires once when a text field loses focus after an edit.
  `Ctrl+Enter` in a `TextArea` calls `on_submit` instead of inserting a newline.
- Mouse input raises `onclick`, `onmousedown`, `onmouseup`, `onmousemove`,
  `onmousedrag` (button held while moving), `onmouseover`/`onmouseout` as the
  topmost node under the pointer changes, and `onwheel` with
  `WheelDelta::lines`. Pressing the left button focuses the deepest focusable
  ancestor. `onmousedrag` is provided by `jsx!` (dioxus-html has no drag event);
  raw `rsx!` code can use the other mouse events.
- Clipboard shortcuts use OSC 52 (`\x1b]52;c;<base64>\x07`) and therefore need
  a terminal that accepts clipboard writes; in tmux enable
  `set-clipboard on`. `Ctrl+C` with no selection still quits.
- A visible `focus_trap` overlay narrows the focus ring to its subtree, blocks
  clicks, hover and wheel events outside it, focuses its `autofocus` target (or
  the first focusable child), and restores the previous focus when it closes.
  `layer` controls sibling paint order, so a toast at `layer:200` stays above a
  modal at `layer:100`.
- `Ctrl+C` quits and restores the terminal. The same cleanup runs on any exit
  from `launch`.

### Terminal size

```rust
let (width, height) = use_terminal_size();
```

The hook subscribes the component to resize events and reports the current
terminal size in cells; `launch` seeds it before the first frame, and
`TuiEvent::Resize` keeps it current when you drive the renderer yourself.

### Text areas

```rust
let mut draft = use_signal(String::new);
jsx! {
    <TextArea
        value={draft()}
        height={Some(5)}
        placeholder="Write a note..."
        on_input={move |text: String| draft.set(text)}
        on_submit={move |_| save(draft())}
    />
}
```

`TextArea` lays out one row per line (plus border and padding), scrolls the
caret into view, and shares selection and clipboard behaviour with `Input`.

## Async data

```rust
let weather = use_fetch::<Forecast>("https://api.open-meteo.com/v1/forecast?...");
let data = weather.suspend()?;           // inside <Suspense>
match &*data.read() { Ok(f) => ..., Err(e) => ... }

let state = use_websocket::<Trade>("wss://example.com/feed"); // Signal<SocketState<T>>
```

`use_fetch` resolves JSON into `T`; `use_websocket` decodes JSON text/binary
messages and exposes `Connecting`/`Open`/`Message`/`Closed`/`Failed`.

## Routing

```rust
rsx! {
    Router {
        routes: routes! {
            "/" => Home,
            "/todo/:id" => TodoPage,
            "/files/*" => Files,
        },
    }
}
```

`use_param("id")` returns the captured segment, `use_query()` parses the query
string, `navigate("/todo/2")` pushes history, and `Link { to: ..., ... }` wraps
both. History is in memory with `back`/`forward` support on `RouterContext`.

## Global state

```rust
static TODOS: Store<Vec<Todo>> = Store::new("todos", Vec::new);

let todos = TODOS.with(|todos| todos.len());   // subscribes the component
TODOS.with_mut(|todos| todos.push(todo));      // rerenders subscribers
```

## Examples

```bash
cargo run -p reactx --example counter
cargo run -p reactx --example todo
cargo run -p reactx --example router
cargo run -p reactx --example scroll     # wheel + keyboard scrolling and clipping
cargo run -p reactx --example editor     # caret editing, scrollable drafts (wheel + click to reopen)
cargo run -p reactx --example overlays   # modal focus trap, Escape and layered toast
cargo run -p reactx --example weather      # needs network
cargo run -p reactx --example websocket    # needs network
cargo run -p reactx --example dashboard    # all data widgets, adapts to terminal height
```

All examples are written with `jsx!`; `rsx!` remains available as the lower
level form.

## Testing

```bash
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo build -p reactx --examples
```

`.github/workflows/ci.yml` runs formatting, clippy with warnings denied, the
test suite and the examples on every push and pull request.

- Unit tests cover style/colour parsing, the node tree, layout measurement,
  focus order and routing.
- `crates/reactx-tui/tests/integration.rs` drives a `VirtualDom` through the
  renderer with Ratatui's `TestBackend`.
- `reactx/tests/widgets.rs` exercises the shipped widgets, store, input submit
  and router navigation headlessly.
- `reactx/tests/framework.rs` covers scroll containers and clipping, caret
  editing and paste, selection/copy/cut, multi-line `TextArea` editing,
  `onchange` on blur, mouse buttons/hover/drag, terminal resizing, and modal
  focus traps, backdrop blocking and layers.
- `reactx/tests/charts.rs` renders every data widget through `TestBackend`,
  checks the `Canvas` marker grids, and drives calendar selection/month
  navigation.
- `reactx/tests/jsx_test.rs` checks that `jsx!` translates nested elements,
  custom components, interpolation, events, fragments, conditionals and
  iterator children into the same behaviour as hand-written `rsx!`.

## Day 1–9 checklist

- [x] **Day 1 — Foundations.** Workspace layout, `NodeTree`/`TuiNode`, style and
  colour parsing, terminal event types.
- [x] **Day 2 — Renderer.** `WriteMutations` bridge for templates, attributes,
  text, placeholders and reordering; node tree maintenance.
- [x] **Day 3 — Layout.** Taffy tree sync, measurement with word/char wrapping,
  absolute rect collection, borders/padding in layout.
- [x] **Day 4 — Events & focus.** Crossterm event translation, focus ring with
  Tab/Shift+Tab, Enter/Space activation, text entry, paste, mouse hit testing,
  Ctrl+C shutdown and terminal restoration.
- [x] **Day 5 — Widgets & theme.** `ThemeProvider`/`use_theme`, Catppuccin Mocha
  default theme, container/text/button/input/checkbox/tabs/list/modal widgets.
- [x] **Day 6 — Router & store.** `routes!`, params, query strings, in-memory
  history, `Link`, and a global `Store<T>` that rerenders subscribers.
- [x] **Day 7 — Async, examples, docs.** `use_fetch`, `use_websocket`,
  `Suspense`, five runnable examples, headless render tests, docs and CI-ready
  commands.
- [x] **Day 8 — Data widgets.** Gauge/LineGauge, Table, BarChart, Sparkline,
  Chart, Canvas (shapes + world map), Scrollbar and an interactive Calendar,
  wired through custom attributes and Ratatui, plus the `dashboard` example.
- [x] **Day 9 — Scroll, editing, overlays.** Clipped scroll containers with
  signed paint offsets and widget cropping, caret-based input editing with
  paste, and layered overlays with a focus trap, backdrop blocking and Escape
  handling, plus the `scroll`, `editor` and `overlays` examples.
- [x] **Day 10 — Rich input and mouse.** Mouse down/up/move/drag and hover
  events, selection with `Ctrl+A`/`Ctrl+C`/`Ctrl+X` over OSC 52, multi-line
  `TextArea` with `Ctrl+Enter`, `change` on blur, configurable `Canvas` markers,
  `use_terminal_size` with an adaptive dashboard, and CI.

## Known limitations

- Components cannot be named `Box` while using `rsx!` (macro hygiene), hence
  `Container`.
- rstml drops whitespace-only text, so `{a} {b}` renders as `ab`; write the
  space inside adjacent text or a quoted `"..."` child. Apostrophes and
  non-ASCII punctuation (`…`, `—`) in unquoted JSX text are rejected by the Rust
  tokenizer — use ASCII or a quoted child (`{"Delete project…"}`) instead.
- Attribute expressions containing generics (`Event<WheelData>`) must be
  wrapped in braces: `onkeydown={move |event: Event<KeyboardData>| ...}`. An
  unbraced closure with a typed parameter makes rstml see `>` as the end of
  the tag; leaving the type off (`move |event| ...`) also works.
- `onmousedrag` only exists in `jsx!`; `dioxus-html` has no drag event, so raw
  `rsx!` code raises the other mouse events but not this one. Embedders can
  still feed `TuiEvent::MouseDrag` to `handle_event` directly.
- Clipboard writes go through OSC 52 only; terminals that ignore it (or tmux
  without `set-clipboard on`) will not see the copied text.
- The router keeps history in memory only; there is no OS/browser integration.
