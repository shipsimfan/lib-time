use super::TimeZone;

/// A time zone with only an offset
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimpleTimeZone(i16);

impl SimpleTimeZone {
    /// Creates a new [`SimpleTimeZone`] with `offset`
    pub const fn new(offset: i16) -> Self {
        SimpleTimeZone(offset)
    }
}

impl TimeZone for SimpleTimeZone {
    const UTC: Self = SimpleTimeZone::new(0);

    #[cfg(feature = "local")]
    fn local() -> Self {
        #[cfg(feature = "alloc")]
        let (offset, _) = super::get_local_time_zone();

        #[cfg(not(feature = "alloc"))]
        let offset = super::get_local_time_zone();

        SimpleTimeZone::new(offset)
    }

    fn offset(&self) -> i16 {
        self.0
    }
}

impl core::fmt::Display for SimpleTimeZone {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.iso8601().fmt(f)
    }
}
