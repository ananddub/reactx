//! Interactive monthly calendar.

use dioxus::prelude::*;

use super::css;
use crate::theme::use_theme;
use crate::widgets::{style_push, Button};

use self::date::{days_in_month, shift_month, today, weekday_monday0, MONTHS, WEEKDAYS};

mod date;

/// An interactive monthly calendar. Days are focusable cells; Enter/Space or a
/// click reports selection through `on_select`.
#[component]
pub fn Calendar(
    year: i32,
    month: u32,
    #[props(default)] selected_day: Option<u32>,
    #[props(default)] on_select: Option<EventHandler<u32>>,
    #[props(default)] on_month_change: Option<EventHandler<(i32, u32)>>,
    /// Tailwind-style utility classes, applied beneath the inline style.
    #[props(default)]
    class: String,
) -> Element {
    let theme = use_theme();
    let (previous_year, previous_month) = shift_month(year, month, -1);
    let (next_year, next_month) = shift_month(year, month, 1);
    let today = today();

    let mut cells: Vec<Option<u32>> = vec![None; weekday_monday0(year, month, 1) as usize];
    cells.extend((1..=days_in_month(year, month)).map(Some));
    while cells.len() % 7 != 0 {
        cells.push(None);
    }

    let center_style = format!(
        "flex-grow:1;text-align:center;bold:true;color:{}",
        css(theme.accent)
    );
    let title = format!("{} {year}", MONTHS[(month.clamp(1, 12) - 1) as usize]);

    let header_cells: Vec<Element> = WEEKDAYS
        .iter()
        .map(|day| {
            rsx! {
                span {
                    style: format!("width:3;text-align:center;color:{}", css(theme.subtext)),
                    "{day}"
                }
            }
        })
        .collect();

    let mut weeks: Vec<Element> = Vec::new();
    for (week_index, week) in cells.chunks(7).enumerate() {
        let mut day_cells: Vec<Element> = Vec::new();
        for (slot, day) in week.iter().enumerate() {
            match day {
                Some(day) => {
                    let selected = selected_day == Some(*day);
                    let is_today = today == Some((year, month, *day));
                    let mut style = String::new();
                    style_push(&mut style, "width", "3");
                    style_push(&mut style, "text-align", "center");
                    if selected {
                        style_push(&mut style, "bg", &css(theme.selection));
                        style_push(&mut style, "color", &css(theme.accent));
                        style_push(&mut style, "bold", "true");
                    } else if is_today {
                        style_push(&mut style, "underline", "true");
                        style_push(&mut style, "color", &css(theme.green));
                    } else {
                        style_push(&mut style, "color", &css(theme.text));
                    }
                    if on_select.is_some() {
                        style_push(&mut style, "focusable", "true");
                    }
                    let select = on_select;
                    let day = *day;
                    day_cells.push(rsx! {
                        span {
                            key: "{year}-{month}-{day}",
                            style: style,
                            onclick: move |_| {
                                if let Some(select) = select {
                                    select.call(day);
                                }
                            },
                            "{day}"
                        }
                    });
                }
                None => {
                    day_cells.push(rsx! {
                        span { key: "blank-{week_index}-{slot}", style: "width:3" }
                    });
                }
            }
        }
        weeks.push(rsx! {
            div { style: "direction:row", {day_cells.into_iter()} }
        });
    }

    let previous = on_month_change;
    let next = on_month_change;

    rsx! {
        div {
            class: class,
            style: format!(
                "direction:column;border:single;border-color:{};padding:0 1;width:23",
                css(theme.border)
            ),
            div { style: "direction:row;align:center",
                if let Some(previous) = previous {
                    Button {
                        variant: "ghost",
                        on_click: move |_| previous.call((previous_year, previous_month)),
                        "‹"
                    }
                } else {
                    span { style: "width:3" }
                }
                span { style: center_style, "{title}" }
                if let Some(next) = next {
                    Button {
                        variant: "ghost",
                        on_click: move |_| next.call((next_year, next_month)),
                        "›"
                    }
                } else {
                    span { style: "width:3" }
                }
            }
            div { style: "direction:row", {header_cells.into_iter()} }
            {weeks.into_iter()}
        }
    }
}
