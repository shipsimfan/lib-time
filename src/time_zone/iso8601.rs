use crate::TimeZone;

/// Displays a time zone in ISO 8601 format
pub struct ISO8601TimeZoneDisplay(i16);

impl ISO8601TimeZoneDisplay {
    /// Creates a new [`ISO8601TimeZoneDisplay`] for `time_zone`
    pub fn new<T: TimeZone>(time_zone: &T) -> Self {
        ISO8601TimeZoneDisplay(time_zone.offset())
    }
}

impl std::fmt::Display for ISO8601TimeZoneDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
