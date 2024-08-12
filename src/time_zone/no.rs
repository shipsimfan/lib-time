use super::TimeZone;

/// No time zone representation, defaults to UTC-like
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoTimeZone;

impl NoTimeZone {
    /// Creates a new [`NoTimeZone`] instance
    pub const fn new() -> Self {
        NoTimeZone
    }
}

impl TimeZone for NoTimeZone {
    const UTC: Self = NoTimeZone;

    #[cfg(feature = "local")]
    fn local() -> Self {
        NoTimeZone
    }

    fn offset(&self) -> i16 {
        0
    }
}

impl core::fmt::Display for NoTimeZone {
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }
}
