# ReactX

A React-style terminal UI framework for Rust. Components, hooks and RSX come
from [Dioxus](https://dioxuslabs.com) 0.6; layout is Flexbox via
[Taffy](https://github.com/DioxusLabs/taffy); drawing and input run through
[Ratatui](https://ratatui.rs) and [Crossterm](https://github.com/crossterm-rs/crossterm);
async work runs on [Tokio](https://tokio.rs).

```toml
# Cargo.toml
[dependencies]
reactx = "0.1.1"      # { path = "reactx" } when developing inside this repo
tokio = { version = "1", features = ["full"] }
```

```rust
use reactx::prelude::*;

fn Dashboard() -> Element {
    let mut count = use_signal(|| 0);
    jsx! {
        <ThemeProvider>
            <Box class="flex flex-col border border-blue-300 rounded p-1 gap-1">
                <span class="text-green-300 bold">Count: {count}</span>
                <div class="flex flex-row gap-1">
                    <Button on_click=move |_| count += 1>+1</Button>
                    <Button variant="danger" on_click=move |_| count -= 1>-1</Button>
                </div>
                <span class="dim">Tab moves focus · Enter activates · Ctrl+C quits</span>
            </Box>
        </ThemeProvider>
    }
}

#[tokio::main]
async fn main() {
    reactx::launch(Dashboard).await;
}
```

`launch` owns the terminal: it enters raw mode, enables mouse and focus
reporting, renders at up to 60 fps while animations run, and restores the
terminal on any exit (including `Ctrl+C`). `restore_terminal()` is exported for
embedders that install their own panic hooks. `launch_inline(app, height)` runs
the app on the normal screen in the bottom `height` rows, so output above it
(such as `<Static>` lines) stays in the terminal scrollback. `launch` installs
a panic hook that restores the terminal, pushes Kitty keyboard-enhancement
flags (key release and alternate keys, disambiguated Escape), and wraps every
frame in a DEC 2026 synchronized update so the scrollback and live view update
atomically.

Everything in this README is available from `reactx::prelude::*`. Every
example and snippet uses the HTML-like `jsx!`, which compiles to Dioxus
elements at build time with the same components, hooks and renderer.

> **How is a component called `Box` possible?** It is written by hand — a
> `BoxProps` struct plus a `Box` function — instead of with `#[component]`.
> The function lives in the value namespace, so the `Box<[Attribute]>` the
> macro expands to still resolves to `std::boxed::Box`.

## Cargo features

`async` and `router` are on by default. Turn them off for a smaller, faster
build and enable only what the app uses:

```toml
[dependencies]
reactx = { version = "0.1", default-features = false }
```

| Feature | Default | Enables |
| --- | --- | --- |
| `async` | yes | `use_fetch`, `use_websocket`, `SocketState`, the async `Suspense` re-export; pulls reqwest and tokio-tungstenite |
| `router` | yes | `Router`, `routes!`, `Link`, `use_param`, `use_query`, `navigate`, `RouterContext`; pulls reactx-router |

With both disabled the dependency tree loses reqwest, hyper, rustls and
tungstenite entirely — roughly 200 unique packages instead of 290.

## JSX syntax

`jsx!` accepts genuine HTML-like markup and compiles it to the same Dioxus
elements the renderer already understands, so components, hooks and the
renderer stay exactly the same:

```rust
use reactx::prelude::*;

fn App() -> Element {
    let mut count = use_signal(|| 0);
    jsx! {
        <Box class="flex flex-col border border-blue-300 rounded p-1 gap-1">
            <span class="text-green-300 bold">Count: {count}</span>
            <Button on_click=move |_| count += 1>+1</Button>
        </Box>
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
| `reactx-ui` | Themed widgets (`Box`, `Text`, `Newline`, `Spacer`, `Button`, `Input`, `TextArea`, `Checkbox`, `Tabs`, `List`, `Modal`, `Animate`), data widgets (`Gauge`, `Table`, `BarChart`, `Sparkline`, `Chart`, `Canvas`, `Scrollbar`, `Calendar`) and `ThemeProvider`/`use_theme` |
| `reactx-async` | `use_fetch`, `use_websocket`, `<Suspense>` |
| `reactx-router` | `Router`, `routes!`, `Link`, `use_location`/`use_param`/`use_query`, in-memory history |
| `reactx-macro` | `jsx!`: compiles HTML-like tags into Dioxus elements with [rstml](https://github.com/rs-tml/rstml) |
| `reactx` | Facade and `reactx::prelude::*`, plus the global `Store<T>` |

## Styling

Styles are ordinary attributes; the renderer maps them onto Taffy and Ratatui.
`style: "key:value;..."` is also accepted and uses the same names.

- **Layout**: `direction` (`row`/`column`/`*-reverse`), `justify`, `align`,
  `align_self`, `align_content`, `flex_grow`, `flex_shrink`, `flex_wrap`,
  `flex_basis`, `order`, `width`, `height`, `min_*`, `max_*`, `aspect_ratio`,
  `padding`, `margin`, `gap`, `position`, `top`/`right`/`bottom`/`left`,
  `display`, `overflow`, `scroll_x`, `scroll_y`. Dimensions accept cells
  (`"10"`), percentages (`"50%"`) or `"auto"`.
- **Visual**: `border` (`single`/`rounded`/`double`/`thick`/`singleDouble`/
  `doubleSingle`/`classic`), `border_color`, `border_top_color`,
  `border_right_color`, `border_bottom_color`, `border_left_color`,
  `border_dim`, `bg`, `color`, `title`, `title_align`, `text_align`, `layer`
  (`z_index`).
- **Text**: `bold`, `italic`, `underline`, `strikethrough`, `dim`, `reversed`
  (`inverse`).
- **Widget state**: `focusable`, `disabled`, `value`, `placeholder`, `label`,
  `checked`, `variant`.
- **Overlays**: `focus_trap` restricts the focus ring and pointer events to a
  subtree, `autofocus` picks the preferred target inside it, and `layer` orders
  siblings (`layer:200` paints above `layer:100`).

Colours accept names (`red`, `bright-green`, ...), `#RGB`, `#RRGGBB`,
`rgb(r, g, b)` and `ansi(n)`.

### Utility classes (Tailwind-style)

`class` accepts a whitespace-separated utility list on any element and on every
widget, including the data widgets and `Link`. Precedence follows CSS:
plain attributes first, then plain utilities, then matching variant utilities,
then the inline `style` attribute.

```rust
let tone = if done { "text-slate-500 line-through" } else { "text-slate-200" };
jsx! {
    <Box class="flex flex-row items-center gap-2 p-2 bg-slate-900 border border-slate-700 rounded">
        <span class={tone}>Ship it</span>
        <span class="grow" />
        <Button class="text-center hover:bg-slate-700" on_click=move |_| {}>Stop</Button>
    </Box>
}
```

Widgets can encode defaults either as classes or as inline styles. `Button`
emits its variant colours, border and padding as Tailwind classes *before* your
`class`, so your utilities override them, and it paints no background unless
you add one. The other widgets still encode some defaults as inline styles,
which a class cannot override: `Text` owns `color`; `Input`/`TextArea` own
their border and colour; `List`/`ListItem` own their border/padding/colour. Use
the widget prop for those (`Text { color: ... }`, `Gauge { color: ... }`), and
classes for layout, sizing, spacing, colour on raw elements, and animation.
Raw elements keep the full CSS-like precedence, so `style` beats their classes
when you need a one-off value.

| Group | Utilities |
| --- | --- |
| Layout | `flex`, `block`, `inline`, `inline-block`, `inline-flex`, `hidden`, `flex-row`/`flex-col` (+ `-reverse`), `flex-wrap`/`flex-wrap-reverse`/`flex-nowrap`, `flex-1`, `flex-auto`, `flex-initial`, `flex-none`, `grow`, `shrink`, `grow-<n>`, `shrink-<n>`, `flex-grow-*`, `flex-shrink-*`, `basis-*`, `order-first`/`order-last`/`order-none`/`order-<n>`, `justify-*`, `items-*`, `self-*`, `content-*`, `place-items-*`, `place-content-*`, `place-self-*` |
| Position | `relative`, `static`, `absolute`, `fixed`, `inset-*`, `inset-x-*`, `inset-y-*`, `top-*`, `right-*`, `bottom-*`, `left-*`, `z-<n>`/`z-[n]`/`z-auto`, `overflow-*` (`hidden`, `scroll`, `auto`, ...), `truncate` |
| Spacing | `p-*`, `px-*`, `py-*`, `pt-*`, `pr-*`, `pb-*`, `pl-*`, `ps-*`, `pe-*`, `m-*`, `mx-*`, `my-*`, `mt-*`, `mr-*`, `mb-*`, `ml-*`, `ms-*`, `me-*`, negative `-m*`/`-p*` forms, `gap-*`, `gap-x-*`, `gap-y-*`, `space-x-*`, `space-y-*` |
| Sizing | `w-*`, `h-*`, `size-*`, `min-w-*`, `min-h-*`, `max-w-*`, `max-h-*`; values are cells (`w-20`), fractions (`w-1/2`), `full`, `screen`, `auto`, `fit`, or brackets (`w-[36]`, `h-[50%]`) |
| Paint | `bg-<colour>`, `text-<colour>`, `text-left`/`text-center`/`text-right`/`text-justify`, `border`, `border-none`/`border-0`, `border-single`, `border-double`, `border-2`/`border-thick`, `border-<colour>`, `rounded`, `rounded-sm/md/lg/xl/full`, `rounded-none`, `bold`/`font-bold`, `font-normal`, `italic`/`not-italic`, `underline`/`no-underline`, `line-through`/`strikethrough`/`no-line-through`, `dim`, `reverse` |
| Animation | `animate-<effect>`, `animate-exit-*`, `duration-*`, `delay-*`, `ease-*`, `animate-repeat-*`, `animate-ping-pong`, `animate-index-*`, `animate-stagger-*`, `animate-direction-*`, `animate-color-*`, `animate-gradient-*`, `animate-key-*`, `animate-spin`/`ping`/`pulse`/`bounce`, `animate-none` (see [Animation](#animation)) |

Values: one spacing unit is one terminal cell, so `p-2` is two cells,
`p-0.5` is half a cell and `p-px` is one cell. Arbitrary values go in brackets
(`w-[20]`, `h-[50%]`, `text-[#f38ba8]`, `z-[200]`, `duration-[450ms]`).
Colours use the Tailwind v3 palette — `slate`, `gray`, `zinc`, `neutral`,
`stone`, `red`, `orange`, `amber`, `yellow`, `lime`, `green`, `emerald`, `teal`,
`cyan`, `sky`, `blue`, `indigo`, `violet`, `purple`, `fuchsia`, `pink`, `rose`,
each with shades `50`–`950` — plus `black`, `white`, `transparent` and the
attribute colour syntax (`red`, `bright-green`, `#RGB`, `rgb(...)`, `ansi(n)`).

Variants: `hover:`, `focus:`, `focus-visible:`, `focus-within:`, `disabled:`,
`checked:`, `first:`, `last:`, `odd:`, `even:`, `group-hover:`, `group-focus:`,
and breakpoints `sm:` (40 columns), `md:` (60), `lg:` (80), `xl:` (100),
`2xl:` (120). Stack them (`md:hover:bg-red-500`).

Variants resolve from the current frame: hover follows the pointer target and
its ancestors, `group-hover:` follows any hovered ancestor, focus follows the
focus ring, `checked:` reads the `checked` state (attribute or widget style),
`first:`/`last:`/`odd:`/`even:` use the node's position among its siblings, and
breakpoints use the terminal width. Precedence is plain utilities, then
matching variants, then the inline `style` attribute.

Not mapped to a terminal: CSS `grid`, auto margins (`m-auto` — use
`items-center`/`justify-center`), per-side borders, opacity levels other than
`dim`, shadows, transforms, font sizes/weights other than `bold`, and text
transform (`uppercase`, `capitalize`). Writing those classes is harmless;
unknown tokens are ignored.

### Theme

`ThemeProvider` puts a `Theme` in context for every descendant; without it,
`use_theme()` falls back to Catppuccin Mocha. Widgets read the theme when they
render, so changing the provider's `theme` prop restyles the subtree.

```rust
use reactx::prelude::*;

let custom = Theme { accent: TuiColor::Rgb(0x89, 0xb4, 0xfa), ..Theme::MOCHA };

jsx! {
    <ThemeProvider theme={custom}>
        <Button on_click=move |_| {}>Themed</Button>
    </ThemeProvider>
}
```

| Field | Used for |
| --- | --- |
| `base`, `mantle`, `crust` | page/modal backgrounds (`crust` is the modal backdrop) |
| `surface` | button background, raised panels |
| `border`, `overlay` | default borders and ghost button borders |
| `text`, `subtext` | body text and secondary text |
| `accent`, `focus` | links, focus ring, active tabs, selected rows, gauges |
| `green`, `yellow`, `red`, `peach`, `mauve`, `teal`, `pink` | statuses, charts and user UI |
| `selection` | selected text background |

`Theme` is `Copy`, so it can be passed around freely. `Theme::mocha()` and
`Theme::MOCHA` give the default; `use_theme()` returns a `Theme` you can read in
your own components (`theme.accent.to_css()` produces a `#rrggbb` string).

## Components

All widgets accept `class` and render ordinary elements, so they compose with
raw `div`/`span` nodes and with each other.

| Component | Required props | Optional props |
| --- | --- | --- |
| `ThemeProvider` | — | `theme: Theme`, children |
| `Box` | — | `direction`, `justify`, `align`, `gap: Option<u16>`, `padding: Option<String>`, `border: String`, `border_color: String`, `border_top_color`/`border_right_color`/`border_bottom_color`/`border_left_color`, `border_dim`, `bg`, `color`, `width`, `height`, `aspect_ratio: Option<f32>`, `flex_grow: Option<f32>`, `title: Option<String>`, `aria_label`, `aria_hidden`, `class`, children |
| `Text` | — | `color`, `bg`, `align`, `wrap` (`wrap`, `hard`, `truncate`, `truncate-start`, `truncate-middle`, `truncate-end`), `bold`, `italic`, `underline`, `strikethrough`, `dim`, `inverse`, `aria_label`, `aria_hidden`, `class`, children |
| `Transform` | `transform: Callback<String, String>` | children; rewrites the text its children render |
| `Newline` | — | `count: u16`; inserts line breaks into the surrounding text |
| `Spacer` | — | — (grows to fill the parent's main axis) |
| `Button` | `on_click: EventHandler<MouseEvent>` | `variant` (`primary` default, `success`, `danger`, `ghost`; emitted as overridable classes, no background), `disabled`, `class`, children |
| `Checkbox` | `on_change: EventHandler<bool>` | `checked`, `label`, `class` |
| `Input` | `on_input: EventHandler<String>` | `on_submit: Option<EventHandler<String>>`, `value`, `placeholder`, `label`, `disabled`, `class` |
| `TextArea` | `on_input: EventHandler<String>` | `on_submit`, `value`, `placeholder`, `label`, `height: Option<u16>`, `disabled`, `class` |
| `List` | — | `title: Option<String>`, `class`, children |
| `ListItem` | — | `selected`, `on_click: Option<EventHandler<()>>`, `class`, children |
| `Tabs` | `tabs: Vec<String>`, `on_select: EventHandler<usize>` | `selected: usize`, `class` |
| `Modal` | `open: bool` | `title`, `on_close: Option<EventHandler<()>>`, `class`, children |
| `Animate` | — | `effect`, `duration`, `easing`, `direction`, `color`, `trigger: u64`, `exit`, `repeat`, `ping_pong`, `delay`, `index`, `step`, `class`, children |
| `Static` | `items: Vec<Line<'static>>` | — |
| `Link` | `to: String` | `class`, children |

`Button`/`Checkbox`/`Input` handlers are required because they are controlled
widgets: feed values back from the handler. `ListItem` rows become focusable
when they have `on_click`. `Modal` renders nothing while `open` is false and
should be the last child of the root so it paints above the page.

### Text transforms

`Transform` rewrites the text its children render, like Ink:

```rust
Transform {
    transform: Callback::new(|text: String| text.to_uppercase()),
    "hello"
}
```

Only the text changes: styles, padding and dimensions stay as laid out, the
callback receives plain text (no ANSI sequences), and nested transforms
compose.

### Screen readers

`aria_label` replaces a component's children with the label for assistive
output, and `aria_hidden` removes the component. Enable the mode with
`LaunchOptions { screen_reader: Some(true), .. }`, or set
`REACTX_SCREEN_READER=true` / `INK_SCREEN_READER=true`.
`use_is_screen_reader_enabled()` reads the flag inside components.

## Static output

`<Static>` writes finished lines into the terminal scrollback once, above the
live view. Pair it with `launch_inline`, which only owns the bottom `height`
rows of the screen:

```rust
fn App() -> Element {
    let ticks = use_ticker(Duration::from_millis(700));
    let tick = ticks();
    let items: Vec<Line<'static>> = (1..=tick)
        .map(|n| Line::from(format!("log line {n}")))
        .collect();

    jsx! {
        <Static items={items} />
        <Box class="border rounded p-1">{"live view"}</Box>
    }
}

#[tokio::main]
async fn main() {
    launch_inline(App, 6).await;
}
```

`Static` only emits items past the previous length, so keep the list
append-only. Fullscreen `launch` has no scrollback region to target and drops
queued lines; use `launch_inline` for logs. This is the same mechanism Ink
uses: Ratatui's `Viewport::Inline` plus `Terminal::insert_before`, so the
terminal itself keeps the history. `Line` and `Span` are in the prelude.

## Headless rendering

`render_to_string(app, width, height)` renders a single frame through a Ratatui
`TestBackend` and returns the screen text, with trailing spaces and blank lines
trimmed — handy for snapshots, logs and assertions:

```rust
let screen = render_to_string(App, 60, 16);
assert!(screen.contains("Count: 0"));
```

## Rendering options

`launch_with(app, options)` and `launch_inline_with(app, height, options)`
accept `LaunchOptions`:

| Field | Default | Meaning |
| --- | --- | --- |
| `exit_on_ctrl_c: bool` | `true` | whether `Ctrl+C` quits; when false your `use_input` handler sees it |
| `kitty_keyboard: bool` | `true` | push the kitty keyboard protocol |
| `screen_reader: Option<bool>` | env-detected | aria output mode |
| `max_fps: Option<f64>` | `Some(30.0)` | frame cap; `None` paints on every change |
| `interactive: Option<bool>` | `stdout().is_terminal()` | when false the app renders headlessly and prints the final frame |
| `on_render: Option<RenderCallback>` | `None` | called after each frame with `RenderMetrics { render_time, frame_time }` |

The `AppHandle` from `use_app()` offers the Ink `Instance` actions: `exit()`,
`unmount()`, `cleanup()`, `clear()`, plus the async
`wait_until_render_flush()` and `wait_until_exit()`.

Without a TTY (CI, piped output) the loop paints into a `TestBackend` sized by
`$COLUMNS`/`$LINES` (80×24 by default) and writes the final frame to stdout on
exit; end such a run from inside the app with `use_app().exit()`.

## Data widgets

Gauges, tables, charts, canvas, scrollbars and the calendar share the same
element tree: each component encodes its props into custom attributes, Taffy
reserves a rectangle (sensible default heights included), and the painter calls
the matching Ratatui widget.

```rust
jsx! {
    <div class="flex flex-col gap-1 h-[14]">
        <Gauge class="grow" value={0.82} label="82% Synced" border="rounded" title="Progress" />
        <Gauge value={45.0} use_line={true} />
    </div>

    <Table
        class="grow"
        headers={vec!["PID".into(), "Process".into(), "CPU".into()]}
        rows={rows}
        widths={Some(vec![6, 18, 8])}
        selected_row={Some(selected())}
        title="Processes"
    />

    <BarChart class="h-[5]" data={vec![("Rust".into(), 95), ("Go".into(), 80)]} horizontal={true} bar_width={Some(1)} />
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

| Widget | Props |
| --- | --- |
| `Gauge` | `value: f64` (0–1), `label`, `use_line`, `color`, `bg`, `title`, `border`, `class` |
| `Sparkline` | `data: Vec<u64>`, `max: Option<u64>`, `color`, `title`, `border`, `class` |
| `BarChart` | `data: Vec<(String, u64)>`, `bar_width: Option<u16>`, `bar_gap: Option<u16>`, `horizontal`, `max: Option<u64>`, `color`, `title`, `border`, `class` |
| `Table` | `headers: Vec<String>`, `rows: Vec<Vec<String>>`, `widths: Option<Vec<u16>>`, `selected_row: Option<usize>`, `title`, `border`, `highlight_color`, `class` |
| `Chart` | `datasets: Vec<ChartDataset>`, `x_bounds: (f64, f64)`, `y_bounds: (f64, f64)`, `x_labels: Option<Vec<String>>`, `y_labels: Option<Vec<String>>`, `title`, `border`, `class` |
| `Canvas` | `x_bounds`, `y_bounds`, `marker` (`braille`/`dot`/`block`/`halfblock`), `shapes: Vec<CanvasShape>`, `title`, `border`, `class` |
| `Scrollbar` | `position: usize`, `content_length: usize`, `orientation: Option<String>`, `thumb_color`, `track_color`, `class` |
| `Calendar` | `year: i32`, `month: u32`, `selected_day: Option<u32>`, `on_select: Option<EventHandler<u32>>`, `on_month_change: Option<EventHandler<(i32, u32)>>`, `class` |

`ChartDataset` has `name`, `data: Vec<(f64, f64)>`, `color: Option<String>` and
`marker` (`dot` default, `braille` or `block`); `ChartDataset::new(name, data)`
fills the optional fields. `CanvasShape` is one of `Line { x1, y1, x2, y2 }`,
`Points { coords }`, `Rect { x, y, width, height }`, `Circle { x, y, radius }`,
`Print { x, y, text }` or `WorldMap {}`; every variant takes `color:
Option<String>`.

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
jsx! {
    <div
        class="flex flex-col h-10 overflow-hidden px-1 border"
        style="scroll-y:{offset}"
        onwheel=move |event| {
            let dy = event.delta().strip_units().y as i32;
            offset.set((offset() as i32 + dy).clamp(0, max) as u16);
        }
    >
        {rows.iter().map(|row| jsx! {
            <span key="{row.id}" class="even:bg-slate-800">{row.label}</span>
        })}
    </div>
}
```

`title` and the `scroll-x`/`scroll-y` offsets have no Tailwind class yet; use
them on the same `style` attribute as any layout classes
(`class="flex flex-col h-10 overflow-hidden px-1" style="title:files;scroll-y:{offset}"`).
`even:`/`odd:` variants make striped rows without a helper component.

Data widgets are cropped the same way: a scrolled `Table` or `Chart` keeps its
row geometry instead of restarting from the first row. Wheel events arrive as
`TuiEvent::Scroll`, bubble to the nearest `onwheel` handler and carry line
deltas (`delta_x`, `delta_y`) plus the held modifiers (`event.modifiers()`).

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
  topmost node under the pointer changes, `onmouseenter`/`onmouseleave`
  (non-bubbling, delivered to every ancestor that listens, as in the DOM), and
  `onwheel` with `WheelDelta::lines`. Left clicks focus the deepest focusable
  ancestor. Two left clicks within 400 ms at the same spot raise
  `ondoubleclick`; the right button raises `oncontextmenu` and the middle
  button only `onmousedown`. Every mouse payload reports
  `event.modifiers()`, so `Ctrl`/`Alt`/`Shift`-clicks are distinguishable.
  `onmousedrag` is provided by `jsx!` (dioxus-html has no drag event); the
  other mouse events work with any markup.
- Key presses raise `onkeydown` and releases raise `onkeyup` on the focused
  node. With nothing focusable, both are delivered to the app's top-level
  elements, so a handler on the root acts like a document-level listener.
  `onkeydown` still fires on the focused node before Tab moves focus.
- Clipboard shortcuts raise `oncopy`/`oncut`/`onpaste` on the focused element
  before the default action runs, so a handler can take over by calling
  `event.prevent_default()`. Clipboard payloads expose their text through
  `event.data().downcast::<TuiClipboardEvent>()`.
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

### Event reference

| Attribute | When it fires | Payload |
| --- | --- | --- |
| `onclick` | left click, `Enter` or `Space` on a button/checkbox/`onclick` node | `MouseEvent` |
| `onmousedown` / `onmouseup` | any button pressed/released; the middle button raises only `onmousedown` | `MouseEvent` with button + modifiers |
| `onmousemove` | pointer moved | `MouseEvent` |
| `onmousedrag` | pointer moved with a button held (jsx! only) | `MouseEvent` |
| `ondoubleclick` | two left clicks within 400 ms at the same cell | `MouseEvent` |
| `oncontextmenu` | right click | `MouseEvent` |
| `onmouseover` / `onmouseout` | topmost node under the pointer changes | `MouseEvent` |
| `onmouseenter` / `onmouseleave` | non-bubbling enter/leave, delivered to every listening ancestor | `MouseEvent` |
| `onwheel` | wheel over the node or a descendant | `WheelData` (`delta_x`, `delta_y` in lines, modifiers) |
| `onkeydown` / `onkeyup` | key pressed/released on the focused node; top-level handlers get unfocused keys | `KeyboardData` (`key()`, `modifiers()`) |
| `oninput` | the value of an `Input`/`TextArea` changes | `FormEvent` |
| `onchange` | a text field loses focus after an edit | `FormEvent` |
| `oncopy` / `oncut` / `onpaste` | clipboard shortcut, before the default action | `TuiClipboardEvent`; `prevent_default()` cancels it |

Mouse payloads expose `event.modifiers()` (`Ctrl`/`Alt`/`Shift`) and the target
node. `onmouseenter`/`onmouseleave` do not bubble; every listening ancestor
receives them, matching the DOM. `onwheel` and `onkeydown` do bubble, so a
handler on a panel can own how its children scroll or respond to keys.

### Terminal size

```rust
let (width, height) = use_terminal_size();
```

The hook subscribes the component to resize events and reports the current
terminal size in cells; `launch` seeds it before the first frame, and
`TuiEvent::Resize` keeps it current when you drive the renderer yourself.

Focus changes use the same pattern: `launch` enables terminal focus reporting,
and `use_terminal_focus()` rerenders the component when the terminal gains or
loses OS focus, so animations can pause while the window is in the background.

Terminal events are drained in batches before each render, with superseded
pointer-motion samples dropped, so a flood of mouse movement cannot starve
clicks and key presses.

### Animation

Four layers are available; pick the smallest one that fits.

**1. Value tweens.** `use_tween` animates a number between renders:

```rust
let smooth = use_tween(target_width, 300u32);   // quad-out, 300 ms
let eased = use_tween(
    target(),
    Tween::new(Duration::from_millis(400)).ease(Interpolation::CubicOut),
);
```

The first render returns the target unchanged; afterwards the value eases
towards it on the ~30 fps ticker and rerenders the component while moving.
Frames stop once it settles, so an idle tween costs nothing. The result plugs
into any prop: gauge values, widths, canvas coordinates, opacity colours.

Two siblings cover other value types:

```rust
let accent = use_color_tween(target_color, 300u32);      // blends RGB channels
let width = use_spring(target_width, Spring::bouncy());  // momentum + overshoot
```

`use_color_tween` takes a ratatui `Color`, maps named/indexed colours to their
xterm RGB values while blending, and returns the exact target once settled.
`use_spring` integrates stiffness/damping/mass, so a moving value keeps its
velocity when the target changes; `Spring::default()` is near critically
damped, `Spring::bouncy()` overshoots and `Spring::stiff()` snaps (`.mass()`
tunes the weight).

**2. Frame-driven animation.** For anything derived from a counter:

```rust
let frame = use_animation_frame();                  // ~30 fps ticker
let slow = use_ticker(Duration::from_millis(120));  // custom cadence
```

Reading the returned `Signal<u64>` inside a component subscribes it to ticks,
so spinner glyphs, canvas shapes and marquee offsets all rerender every frame.
The ticker stops when the component unmounts and runs on the tokio runtime that
`launch` sets up.

**3. Wrapping any subtree.** `<Animate>` plays a tachyonfx effect clipped to
the wrapper's rectangle on mount and again whenever `trigger` changes:

```rust
let mut replay = use_signal(|| 0u64);
jsx! {
    <Animate effect="sweep-in" direction="up" duration={500} trigger={replay()} class="w-full">
        <Table headers={headers} rows={rows} />
    </Animate>
    <Animate effect="dissolve" exit="slide-out" duration={350} index={index as u32} step={60}>
        <Text>{line}</Text>
    </Animate>
    <Button on_click=move |_| replay += 1>Replay</Button>
}
```

The same attributes work on plain elements through the `style` channel:

```rust
jsx! {
    <div style="animate:dissolve;animate-duration:400;animate-easing:linear">
        ...
    </div>
}
```

| Attribute | Meaning |
| --- | --- |
| `animate` | `fade`, `fade-out`, `dissolve`, `coalesce`, `slide-in`, `slide-out`, `sweep-in`, `sweep-out`, `explode`, `hsl-shift`, `glitch`; a comma-separated sequence (`"fade-in, hsl-shift"`); or a name from `register_shader` |
| `animate-exit` | effect name(s) replayed over the node's last painted cells when it leaves the tree (default duration `400`) |
| `animate-duration` | milliseconds (default `600`) |
| `animate-easing` | `linear`, `quad-*`, `cubic-*`, `sine-*`, `expo-*`, `back-*`, `bounce-*`, `elastic-*`, `circ-*`, `reverse` (default `quad-out`) |
| `animate-direction` | `left`/`right`/`up`/`down` for slide and sweep (default `left`) |
| `animate-color` | fill for fade/slide/sweep, any colour syntax (default `black`) |
| `animate-gradient` | gradient width in cells for slide/sweep (default `8`) |
| `animate-key` | change this value to replay the effect (`<Animate trigger>` uses it) |
| `animate-repeat` | `2`, `3x` or `forever` (an effect that never ends keeps the ~60 fps loop alive) |
| `animate-ping-pong` | `1` plays the sequence forwards, then backwards |
| `animate-delay` | wait this many milliseconds before starting |
| `animate-index` + `animate-stagger` | stagger siblings: `index × stagger` milliseconds are added to the delay (default stagger `100`) |

Node effects only touch the node's visible rectangle (scroll offsets and
ancestor clipping included), are advanced by the renderer after the nodes are
painted, and disappear with the node. A finished effect triggers one clean
repaint so no tinted cells are left behind.

Exit animations replay the cells the node had on its last painted frame, so a
removed row fades out as the content it used to show; the freed area is
repainted when the effect finishes. Unknown effect names resolve through a
per-thread registry, so any tachyonfx shader can be used as an `<Animate>`
effect or `animate` value:

```rust
register_shader("sparkle", |_area| {
    fx::dissolve((400, Interpolation::SineIn))
});
```

Every attribute above is also a utility class, so animations compose with the
Tailwind-style API:

```rust
jsx! {
    <Box class="animate-fade duration-300 ease-quad-out">...</Box>
    <Animate class="animate-dissolve animate-exit-slide-out animate-repeat-2">...</Animate>
    <span class="animate-slide-in animate-direction-up animate-color-red-500">...</span>
    <span class="animate-pulse duration-800">...</span> // loops forever
}
```

`duration-*`, `delay-*` and `ease-*` work without the `animate-` prefix
(`ease-in`/`ease-out`/`ease-in-out` map to the cubic curves). Arbitrary values
go in brackets — `duration-[450ms]`, `animate-color-[#f38ba8]` — and sequences
must omit spaces: `animate-[fade-in,dissolve]`. The inline `style` attribute
still wins over classes.

**4. Whole-screen transitions.** `screen_effects()` post-processes the entire
painted frame with [tachyonfx](https://github.com/ratatui/tachyonfx), the
ratatui-org effects library:

```rust
use reactx::prelude::*;

screen_effects().play(fx::fade_from_fg(Color::Black, (500, Interpolation::QuadOut)));
screen_effects().play(fx::sequence(&[
    fx::dissolve((300, Interpolation::Linear)),
    fx::coalesce((300, Interpolation::QuadIn)),
]));
```

It can be called from any event handler (or once from a `use_hook` for an
entrance effect). The renderer advances every active effect with the frame
delta after painting, drops finished effects and repaints once more, and
`launch` keeps rendering at ~60 fps while effects are running. Effects compose
with `fx::sequence`, `fx::parallel`, `fx::repeat` and the rest of the API,
re-exported as `tachyonfx`. See the `animation` example.

### Text areas

```rust
let mut draft = use_signal(String::new);
jsx! {
    <TextArea
        class="grow"
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

## Hooks

ReactX re-exports Dioxus hooks (`use_signal`, `use_memo`, `use_hook`,
`use_context_provider`, ...) and adds the terminal-specific ones below. Every
hook subscribes the calling component only to the state it reads.

| Hook | Returns | Purpose |
| --- | --- | --- |
| `use_terminal_size()` | `(u16, u16)` | current width/height in cells; rerenders on resize |
| `use_window_size()` | `(u16, u16)` | Ink-style alias for `use_terminal_size()` |
| `use_app()` | `AppHandle` | `use_app().exit()` stops the loop and restores the terminal |
| `use_terminal_focus()` | `bool` | whether the OS terminal has focus; rerenders on focus changes |
| `use_animation_frame()` | `Signal<u64>` | ~30 fps ticker; reading it rerenders every frame |
| `use_ticker(Duration)` | `Signal<u64>` | the same ticker at a custom cadence |
| `use_tween(target, impl Into<Tween>)` | `f64` | ease a number towards `target` |
| `use_spring(target, Spring)` | `f64` | spring-physics number with momentum/overshoot |
| `use_color_tween(target, impl Into<Tween>)` | `Color` | blend an RGB colour towards `target` |
| `use_fetch::<T>(url)` | `Resource<Result<T, String>>` | JSON `GET`; call `.suspend()?` in `<Suspense>` |
| `use_websocket::<T>(url)` | `Signal<SocketState<T>>` | JSON WebSocket stream |
| `use_param(name)` | `Option<String>` | a captured route segment (`/todo/:id`) |
| `use_query()` | `Query` | parsed query string (`get`, `get_as`, `iter`, `is_empty`) |
| `use_location()` | `String` | the current path |
| `use_router()` | `RouterContext` | navigation + history |
| `use_theme()` | `Theme` | the nearest `ThemeProvider` theme or Mocha |
| `use_input(handler)` | — | global key handler `(input, TuiKeyEvent)`, runs before focused widgets |
| `use_input_when(active, handler)` | — | `use_input` with an active switch |
| `use_paste(handler)` | — | bracketed-paste handler |
| `use_paste_when(active, handler)` | — | `use_paste` with an active switch |
| `use_focus(id)` | `bool` | whether element `id` is focused; also joins it to the focus ring |
| `use_focus_manager()` | `FocusManagerHandle` | `focus_next`, `focus_previous`, `focus`, `blur`, `enable_focus`, `disable_focus`, `active_id` |
| `use_cursor()` | `CursorHandle` | `set(x, y)`, `hide()`, `reset()` |
| `use_box_metrics(id)` | `BoxMetrics` | `width`, `height`, `left`, `top`, `client_width`, `client_height`; rerenders on layout |
| `measure_element(id)` | `Option<BoxMetrics>` | non-subscribing lookup, safe outside render |
| `use_stdin()` | `StdinHandle` | `is_raw_mode_supported()`, `is_raw_mode()`, `set_raw_mode(bool)` |
| `use_stdout()` | `StdoutHandle` | `write(text)` appends above the live view |
| `use_stderr()` | `StderrHandle` | `write(text)` flushes after the terminal is restored |
| `use_is_screen_reader_enabled()` | `bool` | whether screen-reader mode is active |

`Tween` accepts milliseconds (`300u32`) or a `Duration`, and
`.ease(Interpolation::CubicOut)` selects the curve. `Spring` offers
`Spring::default()`, `Spring::bouncy()`, `Spring::stiff()` and
`Spring::new(stiffness, damping)`; `.mass()` adjusts the weight. The value
hooks stop requesting frames once they settle, so idle UI is cheap.

Global key handlers and focus work together:

```rust
#[component]
fn Search() -> Element {
    let mut query = use_signal(String::new);
    let focused = use_focus("search");
    let focus = use_focus_manager();

    use_input(move |input, key| match key.key {
        TuiKey::Escape => focus.blur(),
        TuiKey::Char(_) => query.write().push_str(&input),
        _ => {}
    });

    let border = if focused { "border-green-300" } else { "border-gray-600" };
    jsx! { <span id="search" class={border}>{query}</span> }
}
```

`use_focus("search")` also puts the element in the focus ring even without a
`focusable` attribute, so Tab and `focus_next()` reach it.

## Async data

```rust
#[component]
fn WeatherView(city: String) -> Element {
    let forecast = use_fetch::<Forecast>(format!("...?city={city}"));
    let data = forecast.suspend()?;          // inside <Suspense>
    match &*data.read() {
        Ok(forecast) => jsx! { <span class="text-blue-300 bold">{forecast.summary}</span> },
        Err(error) => jsx! { <span class="text-rose-400">{"Weather unavailable: {error}"}</span> },
    }
}

jsx! {
    <Suspense>
        <WeatherView city="Berlin" />
    </Suspense>
}
```

`use_fetch` resolves JSON into `T`; while pending, `.suspend()?` suspends the
component and `<Suspense>` paints nothing for it. `use_websocket` decodes JSON
text/binary messages into `T` and exposes
`Connecting`/`Open`/`Message(T)`/`Closed(reason)`/`Failed(error)`. Both hooks
run on `launch`'s Tokio runtime and drop their task when the component
unmounts.

## Routing

```rust
jsx! {
    <Router routes={routes! {
        "/" => Home,
        "/todo/:id" => TodoPage,
        "/files/*" => Files,
    }} />
}
```

`use_param("id")` returns the captured segment, `use_query()` parses the query
string (`?q=rust`), and `use_location()` returns the path. `Link` and
`navigate` both push history:

```rust
jsx! {
    <Link to="/todo/2" class="bold">Open todo #2</Link>
    <Button on_click=move |_| navigate("/")>Home</Button>
}
```

History is in memory. `RouterContext` (from `use_router()`) exposes
`location()`, `route()` (`RouteMatch { index, params, query }`), `navigate(to)`,
`back()`, `forward()`, `can_go_back()` and `can_go_forward()`. A route pattern
can capture `:name` segments and end in `*` for the remaining path (exposed as
`rest`); unmatched paths render the router's `fallback` (or a 404 line).

## Global state

```rust
static TODOS: Store<Vec<Todo>> = Store::new("todos", Vec::new);

let count = TODOS.with(|todos| todos.len());   // subscribes the component
TODOS.with_mut(|todos| todos.push(todo));      // rerenders subscribers
TODOS.set(vec![]);                             // replace + rerender
let snapshot = TODOS.peek();                   // read without subscribing
```

`Store<T>` is `Copy` and keyed by a unique name; the constructor runs once per
Dioxus runtime. Use it for app-wide state that several components read;
`use_signal` remains the right choice for component-local state.

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
cargo run -p reactx --example tailwind     # Tailwind-style classes, animation, hover/focus/breakpoints
cargo run -p reactx --example animation    # tweens, springs, <Animate> and screen effects
cargo run -p reactx --example static_log   # <Static> scrollback above an inline viewport
```

All examples are written with `jsx!` and styled with Tailwind-style utility
classes. Each example stays under 200 lines and focuses on one feature.

| Example | Shows |
| --- | --- |
| `counter` | signals, buttons, keyboard activation, Tailwind layout |
| `todo` | controlled `Input`, `Checkbox`, `List`, filtering |
| `router` | `routes!`, `Link`, `use_param`, `navigate` |
| `scroll` | wheel + keyboard scrolling with clipped descendants |
| `editor` | caret editing, selection, scrollable drafts, live derived values |
| `overlays` | modal focus trap, Escape/backdrop close, layered toast |
| `weather` | `use_fetch` + `Suspense` over a real HTTP API |
| `websocket` | `use_websocket` over a public trade stream |
| `dashboard` | every data widget, adaptive to terminal height |
| `tailwind` | breakpoints, hover/focus/disabled variants, computed classes |
| `animation` | ticker, tweens/springs, `<Animate>`, screen effects |
| `static_log` | append-only `<Static>` output above an inline viewport |

## Testing

```bash
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo build -p reactx --examples
```

- Unit tests cover style/colour parsing, the node tree, layout measurement,
  focus order and routing.
- `crates/reactx-tui/tests/integration.rs` drives a `VirtualDom` through the
  renderer with Ratatui's `TestBackend`.
- `crates/reactx-tui/src/animation/tests.rs` drives the animation ticker
  through the same loop `launch` uses and checks the frame counter advances
  without any input.
- `crates/reactx-tui/tests/animation.rs` checks that `animate*` attributes tint
  only their node's rectangle, restore the screen when done, replay on
  `animate-key`, and that `use_tween` reaches its target without input.
- `crates/reactx-tui/tests/exit.rs` removes an annotated node and checks the
  exit ghost, then exercises sequence/ping-pong/repeat and stagger delays.
- `crates/reactx-tui/tests/motion.rs` drives `use_spring` and
  `use_color_tween` to their targets and plays a `register_shader` effect.
- `crates/reactx-tui/src/effects/tests.rs` queues a `tachyonfx` fade, checks
  it touches the painted frame, and verifies the screen is restored and
  painting stops when the effect finishes.
- `reactx/tests/widgets/` exercises the shipped widgets, store, input submit,
  router navigation and the `<Animate>` wrapper headlessly.
- `reactx/tests/framework.rs` covers scroll containers and clipping, caret
  editing and paste, selection/copy/cut, multi-line `TextArea` editing,
  `onchange` on blur, mouse buttons/hover/drag, terminal resizing, text
  truncation via `render_to_string`, `Newline`/`Spacer` flow, and modal
  focus traps, backdrop blocking and layers.
- `reactx/tests/charts.rs` renders every data widget through `TestBackend`,
  checks the `Canvas` marker grids, and drives calendar selection/month
  navigation.
- `reactx/tests/jsx_test.rs` checks that `jsx!` translates nested elements,
  custom components, interpolation, events, fragments, conditionals and
  iterator children into the same behaviour as the equivalent hand-written
  element trees.

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
  `Suspense`, five runnable examples, headless render tests and docs.
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
  `use_terminal_size` with an adaptive dashboard.
- [x] **Day 11 — Full event parity.** `onkeyup`, document-level key handling
  without focus, mouse modifiers on every pointer payload, `ondoubleclick`,
  `oncontextmenu`, and non-bubbling `onmouseenter`/`onmouseleave`.
- [x] **Day 12 — Loop and clipboard polish.** Batched event handling with
  pointer-motion coalescing, `oncopy`/`oncut`/`onpaste` with `prevent_default`,
  and terminal focus tracking through `use_terminal_focus`.
- [x] **Day 13 — Animation.** `use_animation_frame`/`use_ticker` rerender
  components on a timer that stops on unmount; `use_tween`, `use_spring` and
  `use_color_tween` animate numbers and colours; `<Animate>` and `animate*`
  attributes play effects clipped to any node, including sequences, repeats,
  ping-pong, staggered delays, exit animations and custom shaders; and
  `tachyonfx` screen effects (fade, dissolve, slide-in) post-process painted
  frames while `launch` keeps painting until they finish. The `animation`
  example shows a spinner, gauges, a bouncing canvas node, a scrolling marquee,
  a colour-tweened accent and clickable transitions.
- [x] **Day 14 — Tailwind parity.** Full utility coverage (layout, box, paint,
  values, logical edges, numbered grow/shrink, arbitrary `z-[n]`), state and
  structural variants (`hover`, `focus`, `disabled`, `checked`, `first`, `last`,
  `odd`, `even`, `group-*`), responsive breakpoints, and `class` support on
  every widget, data widget and `Link`.
- [x] **Day 15 — Examples and docs.** Every example is written with `jsx!` and
  styled with utility classes, and this README documents widgets, data widgets,
  hooks, events, theming, routing, stores and animation.

## Known limitations

- rstml drops whitespace-only text, so `{a} {b}` renders as `ab`; write the
  space inside adjacent text or a quoted `"..."` child. Apostrophes and
  non-ASCII punctuation (`…`, `—`) in unquoted JSX text are rejected by the Rust
  tokenizer — use ASCII or a quoted child (`{"Delete project…"}`) instead.
- Attribute expressions containing generics (`Event<WheelData>`) must be
  wrapped in braces: `onkeydown={move |event: Event<KeyboardData>| ...}`. An
  unbraced closure with a typed parameter makes rstml see `>` as the end of
  the tag; leaving the type off (`move |event| ...`) also works.
- `onmousedrag` only exists in `jsx!`; `dioxus-html` has no drag event, so
  the lower-level form raises the other mouse events but not this one.
  Embedders can still feed `TuiEvent::MouseDrag` to `handle_event` directly.
- Clipboard writes go through OSC 52 only; terminals that ignore it (or tmux
  without `set-clipboard on`) will not see the copied text.
- The router keeps history in memory only; there is no OS/browser integration.
- `Transform` rewrites text only. It cannot change colours, borders or layout,
  and ANSI escape sequences in its output are printed literally.
- Ink's stream-override `RenderOptions` (`stdin`/`stdout`/`stderr`), `debug`,
  `patchConsole` and `incrementalRendering` have no equivalent: ReactX always
  uses the process streams, and Ratatui's diffed buffer already updates
  incrementally. There is also no host-level `rerender`; signal writes drive
  renders instead.
- Screen-reader mode is opt-in through `LaunchOptions::screen_reader` or
  `REACTX_SCREEN_READER=true` and understands only `aria_label` and
  `aria_hidden`, not the full Ink `aria-*` vocabulary.

## License

MIT — see [LICENSE](LICENSE). Every crate in the workspace (`reactx`,
`reactx-core`, `reactx-tui`, `reactx-ui`, `reactx-async`, `reactx-router`,
`reactx-macro`) ships under the same license.
