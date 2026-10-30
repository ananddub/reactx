//! A terminal dashboard that exercises every data widget.
//!
//! Run with: `cargo run -p reactx --example dashboard`. The layout adapts to
//! the terminal height so the chart/canvas row stays visible on short screens;
//! 100x32 shows everything comfortably. Buttons drive the gauge, table
//! selection, chart scroll and calendar; days are clickable and focusable.

#![allow(non_snake_case)]

mod data;

use reactx::prelude::*;

fn App() -> Element {
    let (_, terminal_height) = use_terminal_size();
    let top_height = (terminal_height as i32 - 15).clamp(8, 14);
    let mut progress = use_signal(|| 0.62f64);
    let mut selected = use_signal(|| 1usize);
    let mut day = use_signal(|| 15u32);
    let mut year = use_signal(|| 2026i32);
    let mut month = use_signal(|| 10u32);
    let mut scroll = use_signal(|| 12usize);
    let mut samples =
        use_signal(|| vec![12u64, 18, 9, 24, 30, 22, 35, 28, 40, 33, 45, 38, 30, 42, 36]);

    let label = format!("{:.0}% synced", progress() * 100.0);
    let headers = data::headers();
    let rows = data::rows();
    let (latency, errors) = data::datasets();
    let shapes = data::shapes();

    jsx! {
        <ThemeProvider>
            <Box class="flex flex-col p-1 gap-1 h-full">
                <div class="flex flex-row gap-1 items-center">
                    <span class="text-blue-300 bold">ReactX Dashboard</span>
                    <span class="grow" />
                    <Button on_click=move |_| {
                        progress.set((progress() + 0.08).min(1.0));
                        let mut data = samples();
                        data.remove(0);
                        data.push(20 + (progress() * 60.0) as u64);
                        samples.set(data);
                    }>Sync +</Button>
                    <Button variant="danger" on_click=move |_| progress.set((progress() - 0.08).max(0.0))>Sync -</Button>
                    <Button variant="ghost" on_click=move |_| selected.set((selected() + 1) % data::PROCESSES.len())>Next row</Button>
                    <Button variant="ghost" on_click=move |_| scroll.set((scroll() + 7) % 100)>Scroll</Button>
                </div>

                <Box class={format!("flex flex-row gap-1 h-[{top_height}]")}>
                    <Box class="flex flex-col gap-1 w-[36]">
                        <Gauge value={progress()} label={label} border="rounded" title="Sync progress" />
                        <Gauge value={progress()} use_line=true />
                        <Sparkline data={samples()} color="#f9e2af" title="CPU history" border="single" />
                    </Box>
                    <Box class="flex flex-col gap-1 grow">
                        <BarChart
                            data={vec![
                                ("Rust".to_string(), 95u64),
                                ("TypeScript".to_string(), 85),
                                ("Python".to_string(), 62),
                            ]}
                            bar_width={Some(1)}
                            bar_gap={Some(0)}
                            horizontal=true
                            color="#89b4fa"
                            title="Runtime benchmarks"
                            border="single"
                        />
                        <Table
                            headers={headers}
                            rows={rows}
                            widths={Some(vec![6, 18, 8, 10])}
                            selected_row={Some(selected())}
                            title="Processes"
                        />
                    </Box>
                    <Scrollbar position={scroll()} content_length={100usize} orientation="vertical-right" />
                </Box>

                <Box class="flex flex-row gap-1 items-start grow">
                    <Box class="flex flex-col grow-[2] h-full">
                        <Chart
                            datasets={vec![latency, errors]}
                            x_bounds={(0.0, 12.0)}
                            y_bounds={(0.0, 60.0)}
                            x_labels={Some(vec!["0s".to_string(), "6s".to_string(), "12s".to_string()])}
                            title="Latency (ms)"
                            border="single"
                        />
                    </Box>
                    <Box class="flex flex-col grow h-full">
                        <Canvas
                            x_bounds={(0.0, 80.0)}
                            y_bounds={(0.0, 20.0)}
                            shapes={shapes}
                            title="Global nodes"
                            border="single"
                        />
                    </Box>
                    <Calendar
                        year={year()}
                        month={month()}
                        selected_day={Some(day())}
                        on_select=move |value| day.set(value)
                        on_month_change=move |(next_year, next_month)| {
                            year.set(next_year);
                            month.set(next_month);
                        }
                    />
                </Box>

                <span class="dim">{"Click a day, Tab through buttons, Ctrl+C to quit."}</span>
            </Box>
        </ThemeProvider>
    }
}

#[tokio::main]
async fn main() {
    launch(App).await;
}
