use crate::TimeZone;

/// Displays a time zone in ISO 8601 format
pub struct TimeZoneISO8601Display(i16);

impl TimeZoneISO8601Display {
    /// Creates a new [`ISO8601TimeZoneDisplay`] for `time_zone`
    pub fn new<T: TimeZone>(time_zone: &T) -> Self {
        TimeZoneISO8601Display(time_zone.offset())
    }
}

impl core::fmt::Display for TimeZoneISO8601Display {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.0 == 0 {
            return f.write_str("Z");
        }

        let sign = self.0.is_positive();
        let hour = self.0.abs() / 60;
        let minute = self.0.abs() % 60;

        write!(
            f,
            "{}{:02}:{:02}",
            if sign { '+' } else { '-' },
            hour,
            minute
        )
    }
}
