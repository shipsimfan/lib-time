use super::DateTime;
use crate::{SimpleTimeZone, TimeZone};

/// Displays [`DateTime`]s in ISO 8601 format
pub struct DateTimeISO8601Display<'a, T: TimeZone = SimpleTimeZone>(&'a DateTime<T>);

impl<'a, T: TimeZone> DateTimeISO8601Display<'a, T> {
    /// Creates a new [`DateTimeISO8601Display`]
    pub(super) const fn new(date_time: &'a DateTime<T>) -> Self {
        DateTimeISO8601Display(date_time)
    }
}

impl<'a, T: TimeZone> core::fmt::Display for DateTimeISO8601Display<'a, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}{}",
            self.0.year,
            self.0.month + 1,
            self.0.day,
            self.0.hour,
            self.0.minute,
            self.0.second,
            self.0.millisecond,
            self.0.time_zone.iso8601()
        )
    }
}
