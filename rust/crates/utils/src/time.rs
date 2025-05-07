/*
Copyright (c) 2023 Ade M Ramdani <qcynaut@gmail.com>

This software is proprietary and licensed to MyRTS under the terms of the Closed-Source Software License for Freelancers, which is available at https://dictionary.cambridge.org/us/dictionary/english/license.

MyRTS owns all right, title, and interest in and to the software, including all intellectual property rights therein.
MyRTS may use the software for any purpose, including commercial use.
MyRTS may modify the software, but only for their own internal use.
MyRTS may not distribute the software or any modified versions of the software to third parties.
MyRTS may not reverse engineer the software.
MyRTS may not create derivative works from the software.

MyRTS agrees to credit you as the developer of the software in all promotional materials and documentation for the software.

If MyRTS violates any of these terms, their license to use the software will automatically terminate.
*/

use chrono::{Datelike, Duration, NaiveDateTime};

/// Get now timestamp.
pub fn now() -> chrono::NaiveDateTime {
    chrono::Utc::now().naive_utc()
}

/// Get duration in minute.
pub fn minute(m: i64) -> chrono::Duration {
    chrono::Duration::minutes(m)
}

/// Add number of year into timestamp.
pub fn add_year(timestamp: chrono::NaiveDateTime) -> chrono::NaiveDateTime {
    timestamp + chrono::Duration::days(365)
}

/// Check if timestamp + after is greater than now.
pub fn is_greater_than(timestamp: chrono::NaiveDateTime, after: chrono::Duration) -> bool {
    timestamp + after > now()
}

/// Get current month as number.
pub fn current_month() -> u32 {
    chrono::Utc::now().month()
}

/// Get current year as number.
pub fn current_year() -> u32 {
    chrono::Utc::now().year().try_into().unwrap()
}

/// Get first and last date of month.
pub fn get_first_and_last_date_of_month(now: NaiveDateTime) -> (NaiveDateTime, NaiveDateTime) {
    let first = now.with_day(1).unwrap();
    let next_month = first.with_month(now.month() + 1).unwrap_or_else(|| {
        let next_year = now.with_year(now.year() + 1).unwrap();
        next_year.with_month(1).unwrap()
    });
    let last = next_month - Duration::days(1);
    (first, last)
}

/// Get week number.
pub fn week_no() -> u32 {
    let now = chrono::Local::now().naive_local();
    let (first, last) = get_first_and_last_date_of_month(now);
    let num_from_sunday = first.weekday().number_from_sunday();
    let end = first + Duration::days((7 - num_from_sunday) as i64);
    let i = if now >= first && now <= end {
        1
    } else {
        let num_of_weeks = (last.day() - end.day()) / 7;
        let remaining_days = (last.day() - end.day()) % 7;
        let i = num_of_weeks + if remaining_days > 0 { 1 } else { 0 };
        if now >= end && now <= last {
            i + 1
        } else {
            i
        }
    };
    i
}

/// Check if now is week no `week number`.
/// Since in one month there are 4 weeks, the week number is 1, 2, 3, 4.
pub fn is_week_no(week_no: u32) -> bool {
    let now = chrono::Local::now();
    let week_of_month = now.day() / 7 + 1;
    week_no == week_of_month
}

/// Get day number.
pub fn day_no() -> u32 {
    chrono::Local::now().day()
}

/// Get day of week number.
/// 1 is sunday, 7 is saturday.
pub fn day_of_week() -> u32 {
    let from_sunday = chrono::Local::now().weekday().number_from_sunday();
    from_sunday
}

/// Get time in 24 hour format.
pub fn time_24() -> String {
    chrono::Local::now().format("%H:%M").to_string()
}
