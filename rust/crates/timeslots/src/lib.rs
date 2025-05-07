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

use chrono::{Datelike, Duration};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MDay represent Minute per Day.
#[derive(Clone, Serialize, Deserialize)]
pub struct MDay {
    slots: Vec<u8>,
    cap: usize,
}

impl MDay {
    /// Create a new MDay.
    pub fn new() -> Self {
        Self {
            slots: vec![0; 1440],
            cap: 1440,
        }
    }

    /// check if the given range is filled.
    fn is_filled(&self, start: u32, end: u32) -> bool {
        let range = &self.slots[start as usize..(end + 1) as usize];
        range.contains(&1)
    }

    /// Add schedule, return (filled, overflow, extra).
    pub fn add(&mut self, hour: u32, minute: u32, duration: u32) -> (bool, bool, u32) {
        let start = hour * 60 + minute;
        let overflowed = start + duration > self.cap as u32;
        let extra = if overflowed {
            self.cap as u32 - start
        } else {
            0
        };
        let end = if overflowed {
            self.cap as u32
        } else {
            start + duration
        };
        if self.is_filled(start, end) {
            return (true, false, 0);
        }
        self.slots[start as usize..(end + 1) as usize]
            .iter_mut()
            .for_each(|x| *x = 1);
        (false, overflowed, extra)
    }

    /// Remove schedule.
    pub fn remove(&mut self, hour: u32, minute: u32, duration: u32) {
        let start = hour * 60 + minute;
        let end = start + duration;
        if start >= self.cap as u32 || end >= self.cap as u32 {
            return;
        }
        self.slots[start as usize..(end + 1) as usize]
            .iter_mut()
            .for_each(|x| *x = 0);
    }
}

/// MWeek represent Minute per Week.
#[derive(Clone, Serialize, Deserialize)]
pub struct MWeek {
    slots: Vec<MDay>,
    cap: usize,
}

impl MWeek {
    /// Create a new MWeek.
    pub fn new() -> Self {
        Self {
            slots: vec![MDay::new(); 7],
            cap: 7,
        }
    }

    /// Add schedule, if it's alredy filled, return false.
    pub fn add(&mut self, day: u8, hour: u8, minute: u8, duration: u32) -> bool {
        // 7 is the last day of the week.
        if day > 7 {
            return true;
        }
        let backup = self.slots.clone();
        let (mut filled, overflow, extra) =
            self.slots[day as usize - 1].add(hour as u32, minute as u32, duration);
        if filled {
            return true;
        }
        if overflow {
            filled = self.slots[day as usize].add(0, 0, extra).0;
        }
        if filled {
            self.slots = backup;
            return true;
        }
        false
    }

    /// Remove schedule.
    pub fn remove(&mut self, day: u8, hour: u8, minute: u8, duration: u32) {
        if day > 7 {
            return;
        }
        self.slots[day as usize - 1].remove(hour as u32, minute as u32, duration);
    }
}

/// Once.
/// This is an once schedule (non repeating).
#[derive(Clone, Serialize, Deserialize)]
pub struct Once {
    year: u16,
    month: u8,
    days: HashMap<u8, MDay>,
}

/// TimeSlots.
/// This is a full time slots store for managing schedules.
/// This has size of u8x1440x7x5+(additional slots for non repeating schedules).
#[derive(Clone, Serialize, Deserialize)]
pub struct TimeSlots {
    slots: Vec<MWeek>,
    cap: usize,
    onces: HashMap<u16, HashMap<u8, Vec<MWeek>>>,
}

impl TimeSlots {
    /// Create a new TimeSlots.
    pub fn new() -> Self {
        Self {
            slots: vec![MWeek::new(); 5],
            cap: 5,
            onces: HashMap::new(),
        }
    }

    /// Add schedule, if it's alredy filled, return false.
    pub fn add(&mut self, date: u8, hour: u8, minute: u8, duration: u32) -> bool {
        // 31 is the last day of the month.
        if date > 31 {
            return true;
        }
        // since we have 5 weeks, we need to calculate the week number.
        let div = date / 7;
        let rem = date % 7;
        let week = if rem == 0 { div } else { div + 1 };
        let day = if rem == 0 { 7 } else { rem };
        self.slots[week as usize - 1].add(day, hour, minute, duration as u32)
    }

    /// Remove schedule.
    pub fn remove(&mut self, date: u8, hour: u8, minute: u8, duration: u32) {
        // 31 is the last day of the month.
        if date > 31 {
            return;
        }
        // since we have 5 weeks, we need to calculate the week number.
        let div = date / 7;
        let rem = date % 7;
        let week = if rem == 0 { div } else { div + 1 };
        let day = if rem == 0 { 7 } else { rem };
        self.slots[week as usize - 1].remove(day, hour, minute, duration);
    }

    /// Add schedule, if it's alredy filled, return false.
    pub fn add_week(&mut self, week: u8, day: u8, hour: u8, minute: u8, duration: u32) -> bool {
        self.slots[week as usize - 1].add(day, hour, minute, duration as u32)
    }

    /// Remove week.
    pub fn remove_week(&mut self, week: u8, day: u8, hour: u8, minute: u8, duration: u32) {
        self.slots[week as usize - 1].remove(day, hour, minute, duration);
    }

    /// Add once schedule.
    pub fn add_once(
        &mut self,
        year: u16,
        month: u8,
        date: u8,
        hour: u8,
        minute: u8,
        duration: u32,
    ) -> bool {
        let date = chrono::NaiveDate::from_ymd_opt(year as i32, month as u32, date as u32).unwrap();
        let naive = chrono::NaiveDateTime::new(
            date,
            chrono::NaiveTime::from_hms_opt(hour as u32, minute as u32, 0).unwrap(),
        );
        let (first, last) = {
            let first = naive.with_day(1).unwrap();
            let next_month = first.with_month(naive.month() + 1).unwrap_or_else(|| {
                let next_year = naive.with_year(naive.year() + 1).unwrap();
                next_year.with_month(1).unwrap()
            });
            let last = next_month - Duration::days(1);
            (first, last)
        };
        let week = {
            let num_from_sunday = first.weekday().number_from_sunday();
            let end = first + Duration::days((7 - num_from_sunday) as i64);
            let i = if naive >= first && naive <= end {
                1
            } else {
                let num_of_weeks = (last.day() - end.day()) / 7;
                let remaining_days = (last.day() - end.day()) % 7;
                let i = num_of_weeks + if remaining_days > 0 { 1 } else { 0 };
                if naive >= end && naive <= last {
                    i + 1
                } else {
                    i
                }
            };
            i
        };
        let day = naive.weekday().number_from_sunday();
        let backup = self.slots.clone();
        let filled = self.add_week(week as u8, day as u8, hour, minute, duration);
        if filled {
            self.slots = backup;
            return true;
        }
        self.slots = backup;

        let once_backup = self.onces.clone();
        if let Some(once) = self.onces.get_mut(&year) {
            if let Some(weeks) = once.get_mut(&month) {
                let filled = weeks[week as usize].add(day as u8, hour, minute, duration as u32);
                if filled {
                    self.onces = once_backup;
                    return true;
                } else {
                    return false;
                }
            } else {
                let mut mweek = MWeek::new();
                mweek.add(day as u8, hour, minute, duration as u32);
                once.insert(month, vec![mweek]);
                return false;
            }
        } else {
            let mut mweek = MWeek::new();
            mweek.add(day as u8, hour, minute, duration as u32);
            let mut once = HashMap::new();
            once.insert(month, vec![mweek]);
            self.onces.insert(year, once);
            return false;
        }
    }

    /// Remove once schedule.
    pub fn remove_once(
        &mut self,
        year: u16,
        month: u8,
        date: u8,
        hour: u8,
        minute: u8,
        duration: u32,
    ) {
        let date = chrono::NaiveDate::from_ymd_opt(year as i32, month as u32, date as u32).unwrap();
        let naive = chrono::NaiveDateTime::new(
            date,
            chrono::NaiveTime::from_hms_opt(hour as u32, minute as u32, 0).unwrap(),
        );
        let (first, last) = {
            let first = naive.with_day(1).unwrap();
            let next_month = first.with_month(naive.month() + 1).unwrap_or_else(|| {
                let next_year = naive.with_year(naive.year() + 1).unwrap();
                next_year.with_month(1).unwrap()
            });
            let last = next_month - Duration::days(1);
            (first, last)
        };
        let week = {
            let num_from_sunday = first.weekday().number_from_sunday();
            let end = first + Duration::days((7 - num_from_sunday) as i64);
            let i = if naive >= first && naive <= end {
                1
            } else {
                let num_of_weeks = (last.day() - end.day()) / 7;
                let remaining_days = (last.day() - end.day()) % 7;
                let i = num_of_weeks + if remaining_days > 0 { 1 } else { 0 };
                if naive >= end && naive <= last {
                    i + 1
                } else {
                    i
                }
            };
            i
        };
        let day = naive.weekday().number_from_sunday();
        if let Some(once) = self.onces.get_mut(&year) {
            if let Some(month) = once.get_mut(&month) {
                if let Some(weeks) = month.get_mut(week as usize) {
                    weeks.remove(day as u8, hour, minute, duration as u32);
                }
            }
        }
    }
}

impl TryFrom<String> for TimeSlots {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_slice(value.as_ref()).map_err(|e| e.to_string())
    }
}

impl From<TimeSlots> for String {
    fn from(value: TimeSlots) -> Self {
        serde_json::to_string(&value).unwrap()
    }
}
