use chrono::prelude::*;
use chrono::Duration;
use lazy_static::lazy_static;

use std::fmt;

pub use chrono::{Datelike, NaiveDate};

lazy_static! {
    static ref START_DATE: NaiveDate = NaiveDate::from_ymd(2020, 1, 2);
    static ref CYCLE_LENGTH: Duration = Duration::days(28);
}

/// A representation ICAO defined AIRAC cycle.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct AIRAC(NaiveDate);

impl AIRAC {
    /// Returns the AIRAC cycle valid on the day given
    pub fn from_ymd(y: i32, m: u32, d: u32) -> Self {
        let mut airac_date = START_DATE.clone();
        let target = NaiveDate::from_ymd(y, m, d);
        if y < 2020 {
            // Move backward in time
            loop {
                airac_date -= *CYCLE_LENGTH;
                if airac_date < target {
                    break;
                }
            }
        } else {
            // Move forward in time
            loop {
                if airac_date + *CYCLE_LENGTH > target {
                    break;
                }
                airac_date += *CYCLE_LENGTH;
            }
        }
        Self(airac_date)
    }

    /// Get the current active AIRAC
    pub fn current() -> Self {
        let today = Utc::today().naive_utc();
        Self::from_ymd(today.year(), today.month(), today.day())
    }

    /// Returns the previous AIRAC cycle.
    pub fn previous(&self) -> Self {
        Self(self.0 - *CYCLE_LENGTH)
    }

    /// Returns the next AIRAC cycle.
    pub fn next(&self) -> Self {
        Self(self.0 + *CYCLE_LENGTH)
    }

    /// The date that this AIRAC stared on.
    pub fn starts(&self) -> NaiveDate {
        self.0
    }

    /// The date at which this AIRAC became ineffective.
    /// For the avoidance of doubt, the AIRAC became ineffective as this day
    /// began.
    pub fn ends(&self) -> NaiveDate {
        self.0 + *CYCLE_LENGTH
    }
}

impl fmt::Display for AIRAC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut airac_count = 0;
        let mut a = self.clone();
        loop {
            a = a.previous();
            if a.starts().year() != self.starts().year() {
                break;
            }
            airac_count += 1;
        }
        write!(f, "{}{:02}", self.0.format("%y"), airac_count + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ord() {
        let current = AIRAC::current();
        let prev = current.previous();
        assert!(prev < current);
        let next = current.next();
        assert!(next > current);
    }

    #[test]
    fn test_leap_years() {
        let airac = AIRAC::from_ymd(2020, 12, 31);
        assert_eq!("2014", format!("{}", airac));
    }

    #[test]
    fn test_started_and_ends_correct_pre_2020() {
        let airac = AIRAC::from_ymd(2018, 02, 17);
        assert_eq!(airac.starts(), NaiveDate::from_ymd(2018, 02, 01));
        assert_eq!(airac.ends(), NaiveDate::from_ymd(2018, 03, 01));
    }

    #[test]
    fn test_started_and_ends_correct_post_2020() {
        let airac = AIRAC::from_ymd(2022, 05, 23);
        assert_eq!(airac.starts(), NaiveDate::from_ymd(2022, 05, 19));
        assert_eq!(airac.ends(), NaiveDate::from_ymd(2022, 06, 16));
    }

    #[test]
    fn test_airac_pre2020() {
        let airac = AIRAC::from_ymd(2019, 05, 13);
        assert_eq!("1905", format!("{}", airac));
    }

    #[test]
    fn test_airac_post2020() {
        let airac = AIRAC::from_ymd(2022, 05, 23);
        assert_eq!("2205", format!("{}", airac));
    }
}
