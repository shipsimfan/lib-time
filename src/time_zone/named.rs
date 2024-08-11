use super::TimeZone;
use std::{borrow::Cow, fmt::Display, hash::Hash};

/// A time zone with a name
#[derive(Debug, Clone)]
pub struct NamedTimeZone {
    /// The offset, in minutes, from UTC
    offset: i16,

    /// The name of this time zone
    name: Cow<'static, str>,
}

impl NamedTimeZone {
    /// Creates a new [`NamedTimeZone`]
    pub const fn new(offset: i16, name: &'static str) -> Self {
        NamedTimeZone {
            offset,
            name: Cow::Borrowed(name),
        }
    }

    /// Creates a new [`NamedTimeZone`]
    pub const fn new_owned(offset: i16, name: String) -> Self {
        NamedTimeZone {
            offset,
            name: Cow::Owned(name),
        }
    }

    /// Gets the name of this time zone
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl PartialEq for NamedTimeZone {
    fn eq(&self, other: &Self) -> bool {
        self.offset.eq(&other.offset)
    }
}

impl Eq for NamedTimeZone {}

impl PartialOrd for NamedTimeZone {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NamedTimeZone {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.offset.cmp(&other.offset)
    }
}

impl Hash for NamedTimeZone {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.offset.hash(state)
    }
}

impl Display for NamedTimeZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

impl TimeZone for NamedTimeZone {
    const UTC: Self = NamedTimeZone::new(0, "UTC");

    fn local() -> Option<Self> {
        todo!()
    }

    fn offset(&self) -> i16 {
        self.offset
    }
}
