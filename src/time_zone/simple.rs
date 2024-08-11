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

    fn local() -> Option<Self> {
        todo!()
    }

    fn offset(&self) -> i16 {
        self.0
    }
}

impl std::fmt::Display for SimpleTimeZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iso8601().fmt(f)
    }
}
