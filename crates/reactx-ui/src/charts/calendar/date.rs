//! Civil-date helpers for the calendar, backed by chrono.

use chrono::{Datelike, Local, NaiveDate};

pub(super) const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

pub(super) const WEEKDAYS: [&str; 7] = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];

pub(super) fn days_in_month(year: i32, month: u32) -> u32 {
    let (next_year, next_month) = shift_month(year, month, 1);
    NaiveDate::from_ymd_opt(next_year, next_month, 1)
        .and_then(|first| first.pred_opt())
        .map(|last| last.day())
        .unwrap_or(30)
}

/// Monday-based weekday (0 = Monday) for a civil date.
pub(super) fn weekday_monday0(year: i32, month: u32, day: u32) -> u32 {
    NaiveDate::from_ymd_opt(year, month, day)
        .map(|date| date.weekday().num_days_from_monday())
        .unwrap_or(0)
}

pub(super) fn shift_month(year: i32, month: u32, delta: i32) -> (i32, u32) {
    let index = year * 12 + month as i32 - 1 + delta;
    (index.div_euclid(12), (index.rem_euclid(12) + 1) as u32)
}

/// Today's date in the system's local timezone.
pub(super) fn today() -> Option<(i32, u32, u32)> {
    let now = Local::now().date_naive();
    Some((now.year(), now.month(), now.day()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weekdays_are_monday_based() {
        assert_eq!(weekday_monday0(1970, 1, 1), 3); // Thursday
        assert_eq!(weekday_monday0(2026, 10, 1), 3); // Thursday
        assert_eq!(weekday_monday0(2026, 10, 29), 3); // Thursday
        assert_eq!(weekday_monday0(2026, 11, 1), 6); // Sunday
    }

    #[test]
    fn month_lengths_and_shifts() {
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2026, 2), 28);
        assert_eq!(days_in_month(2026, 10), 31);
        assert_eq!(shift_month(2026, 12, 1), (2027, 1));
        assert_eq!(shift_month(2026, 1, -1), (2025, 12));
    }
}
