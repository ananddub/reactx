use crate::harness::Harness;
use reactx::prelude::*;

fn calendar_app() -> Element {
    let mut selected = use_signal(|| 10u32);
    let mut year = use_signal(|| 2026i32);
    let mut month = use_signal(|| 10u32);
    jsx! {
        <div style="direction:column;width:40">
            <Calendar
                year={year()}
                month={month()}
                selected_day={Some(selected())}
                on_select=move |value| selected.set(value)
                on_month_change=move |(next_year, next_month)| {
                    year.set(next_year);
                    month.set(next_month);
                }
            />
            <span>Selected: {selected}</span>
        </div>
    }
}

#[test]
fn calendar_shows_month_and_weekdays() {
    let harness = Harness::new(calendar_app);
    let screen = harness.screen();
    assert!(screen.contains("October 2026"), "{screen}");
    for day in ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"] {
        assert!(screen.contains(day), "missing {day}; screen:\n{screen}");
    }
    assert!(screen.contains("Selected: 10"), "{screen}");
}

#[test]
fn calendar_click_selects_day() {
    let mut harness = Harness::new(calendar_app);
    harness.click_text("text", "29");
    assert!(
        harness.screen().contains("Selected: 29"),
        "{}",
        harness.screen()
    );
}

#[test]
fn calendar_navigates_months() {
    let mut harness = Harness::new(calendar_app);
    harness.click_text("button", "›");
    assert!(
        harness.screen().contains("November 2026"),
        "{}",
        harness.screen()
    );
    harness.click_text("button", "‹");
    harness.click_text("button", "‹");
    assert!(
        harness.screen().contains("September 2026"),
        "{}",
        harness.screen()
    );
}
