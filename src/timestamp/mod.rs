use crate::{SimpleTimeZone, TimeZone};

/// A point in time
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp<T: TimeZone = SimpleTimeZone> {
    /// The number of nanoseconds from the Unix epoch of this time
    timestamp: i128,

    /// The timezone of this timestamp
    time_zone: T,
}

impl<T: TimeZone> Timestamp<T> {
    /// The timestamp representing the Unix epoch
    pub const UNIX_EPOCH: Self = Timestamp::new_utc(0);

    /// Creates a new [`Timestamp`]
    pub const fn new(timestamp: i128, time_zone: T) -> Self {
        Timestamp {
            timestamp,
            time_zone,
        }
    }

    /// Creates a new timestamp in UTC time zone
    pub const fn new_utc(timestamp: i128) -> Self {
        Timestamp::new(timestamp, T::UTC)
    }

    /// Creates a new timestamp in the local time zone
    #[cfg(feature = "local")]
    pub fn new_local(timestamp: i128) -> Self {
        Timestamp::new(timestamp, T::local())
    }

    /// Gets the current timestamp in UTC
    #[cfg(feature = "now")]
    pub fn now_utc() -> Self {
        todo!()
    }

    /// Gets the current timestamp in the local timezone
    #[cfg(all(feature = "now", feature = "local"))]
    pub fn now_local() -> Self {
        todo!()
    }

    /// Gets the timestamp in nanoseconds
    pub const fn as_nanos(&self) -> i128 {
        self.timestamp
    }

    /// Gets the timestamp in microseconds
    pub const fn as_micros(&self) -> i64 {
        (self.timestamp / 1_000) as i64
    }

    /// Gets the timestamp in microseconds
    pub const fn as_millis(&self) -> i64 {
        self.as_micros() / 1_000
    }

    /// Gets the timestamp in seconds
    pub const fn as_secs(&self) -> i64 {
        self.as_millis() / 1_000
    }

    /// Gets the timestamp in minutes
    pub const fn as_mins(&self) -> i64 {
        self.as_secs() / 60
    }

    /// Gets the timestamp in hours
    pub const fn as_hours(&self) -> i64 {
        self.as_mins() / 60
    }

    /// Gets the timestamp in days
    pub const fn as_days(&self) -> i64 {
        self.as_hours() / 24
    }

    /// Gets the timestamp in weeks
    pub const fn as_weeks(&self) -> i64 {
        self.as_days() / 7
    }

    /// Gets the timestamp in years
    pub const fn as_years(&self) -> i64 {
        self.as_days() / 365
    }

    /// Gets the timezone of this timestamp
    pub const fn time_zone(&self) -> &T {
        &self.time_zone
    }

    /// Changes the timezone of this timestamp, adjusting the time accordingly
    pub fn change_timezone(&mut self, new_timezone: T) {
        let difference = new_timezone.offset() - self.time_zone.offset();

        self.timestamp += (difference as i128) * (1_000 * 1_000 * 1_000 * 60);
        self.time_zone = new_timezone;
    }
}

impl<T: TimeZone> core::fmt::Display for Timestamp<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}{}", self.timestamp, self.time_zone)
    }
}

#[cfg(feature = "std")]
impl<T: TimeZone> From<std::time::SystemTime> for Timestamp<T> {
    fn from(value: std::time::SystemTime) -> Self {
        let duration = value.duration_since(std::time::UNIX_EPOCH).unwrap();
        let timestamp = duration.as_nanos() as i128;
        Timestamp::new_utc(timestamp)
    }
}
