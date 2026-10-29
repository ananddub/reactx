//! Animated widgets, value tweens, `tachyonfx` effects.
//!
//! Run with: `cargo run -p reactx --example animation`. The
//! `use_animation_frame` hook ticks about 30 times per second and rerenders
//! this component, so the spinner, gauges, canvas ball and marquee all move.
//! One gauge is driven by `use_tween` instead, the canvas sits inside an
//! `<Animate>` wrapper that replays a node-scoped sweep, and the buttons queue
//! whole-screen `tachyonfx` effects; `launch` keeps painting until they
//! finish.

#![allow(non_snake_case)]

use std::time::Duration;

use reactx::prelude::*;

const SPINNER: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
const PALETTE: [&str; 6] = [
    "#89b4fa", "#a6e3a1", "#f9e2af", "#f38ba8", "#cba6f7", "#94e2d5",
];
const MARQUEE: &str = " ReactX · a 30 fps ticker rerenders every frame · gauges, spinners and canvas shapes keep moving ";

/// Triangle wave in `0.0..=1.0`, bouncing between the ends every `period`.
fn wave(t: f64, period: f64) -> f64 {
    let phase = (t % period) / period;
    if phase < 0.5 {
        phase * 2.0
    } else {
        2.0 - phase * 2.0
    }
}

fn ball(t: f64) -> (f64, f64) {
    let x = 4.0 + wave(t * 0.9 + 8.0, 120.0) * 68.0;
    let y = 2.0 + wave(t * 0.55 + 40.0, 76.0) * 6.0;
    (x, y)
}

fn css(color: Color) -> String {
    match color {
        Color::Rgb(r, g, b) => format!("#{r:02x}{g:02x}{b:02x}"),
        _ => "white".into(),
    }
}

fn App() -> Element {
    let frame = use_animation_frame();
    let (width, _) = use_terminal_size();
    let mut replay = use_signal(|| 0u64);
    use_hook(|| {
        screen_effects().play(fx::fade_from_fg(
            Color::Black,
            (700, Interpolation::QuadOut),
        ))
    });
    let f = frame();
    let t = f as f64;
    let seconds = t / 30.0;
    let spinner = SPINNER[f as usize % SPINNER.len()];
    let progress = (f % 120) as f64 / 120.0;
    let percent = format!("{:.0}%", progress * 100.0);
    let goal = if (f / 90) % 2 == 0 { 1.0 } else { 0.15 };
    let smooth = use_tween(
        goal,
        Tween::new(Duration::from_millis(450)).ease(Interpolation::CubicOut),
    );
    let status = format!("{spinner} running for {seconds:.1}s");
    let live = (f / 15) % 2 == 0;
    let led = if live { "● live" } else { "○ live" };
    let accent = css(use_color_tween(
        TuiColor::parse(PALETTE[(f / 30) as usize % PALETTE.len()])
            .map(TuiColor::to_ratatui)
            .unwrap_or(Color::White),
        350u32,
    ));
    let sprung = use_spring(progress, Spring::bouncy()).clamp(0.0, 1.0);

    let marquee_width = width.saturating_sub(4) as usize;
    let marquee: String = MARQUEE
        .chars()
        .cycle()
        .skip(f as usize / 2)
        .take(marquee_width)
        .collect();

    let (bx, by) = ball(t);
    let shapes = vec![
        CanvasShape::Line {
            x1: 0.0,
            y1: 9.0,
            x2: 76.0,
            y2: 9.0,
            color: Some("#313244".into()),
        },
        CanvasShape::Points {
            coords: (1..=8).map(|k| ball(t - k as f64 * 2.5)).collect(),
            color: Some("#585b70".into()),
        },
        CanvasShape::Circle {
            x: bx,
            y: by,
            radius: 4.0,
            color: Some(accent.clone()),
        },
    ];

    jsx! {
        <ThemeProvider>
            <Container class="flex flex-col p-1 gap-1 h-full">
                <div class="flex flex-row gap-1 items-center">
                    <span class={format!("bold text-[{accent}]")}>ReactX Animation</span>
                    <span class="grow" />
                    <span class="dim">{format!("frame {f} · {seconds:.1}s")}</span>
                </div>
                <Container class="flex flex-row gap-1">
                    <Container class="flex flex-col grow-[2]">
                        <Animate
                            effect="sweep-in"
                            direction="up"
                            duration={700}
                            color="#313244"
                            trigger={replay()}
                            class="w-full"
                        >
                            <Canvas
                                x_bounds={(0.0, 76.0)}
                                y_bounds={(0.0, 10.0)}
                                shapes={shapes}
                                title="Bouncing node"
                                border="single"
                            />
                        </Animate>
                    </Container>
                    <Container class="flex flex-col gap-1 grow">
                        <Gauge value={progress} label={percent.clone()} border="rounded" title="Cycle" />
                        <Gauge
                            value={sprung}
                            label={format!("{:.0}%", sprung * 100.0)}
                            border="rounded"
                            title="Spring"
                            color={accent.clone()}
                        />
                        <Gauge
                            value={smooth}
                            label={format!("{:.0}% eased", smooth * 100.0)}
                            border="rounded"
                            title="Tween"
                        />
                        <span class={if live { "text-green-300" } else { "text-slate-600" }}>{led}</span>
                        <span class="dim">{status}</span>
                        <div class="flex flex-row gap-1">
                            <Button variant="ghost" on_click=move |_| {
                                screen_effects().play(fx::fade_from_fg(Color::Black, (500, Interpolation::QuadOut)))
                            }>Fade</Button>
                            <Button variant="ghost" on_click=move |_| {
                                screen_effects().play(fx::dissolve((600, Interpolation::Linear)))
                            }>Dissolve</Button>
                            <Button variant="ghost" on_click=move |_| {
                                screen_effects().play(fx::slide_in(
                                    Motion::LeftToRight,
                                    20,
                                    0,
                                    Color::Black,
                                    (600, Interpolation::QuadOut),
                                ))
                            }>Slide</Button>
                            <Button variant="ghost" on_click=move |_| replay += 1>Replay</Button>
                        </div>
                        <span class="dim">Ctrl+C to quit.</span>
                    </Container>
                </Container>
                <span class="dim">{marquee}</span>
            </Container>
        </ThemeProvider>
    }
}

#[tokio::main]
async fn main() {
    launch(App).await;
}
