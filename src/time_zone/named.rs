use super::TimeZone;
use core::{fmt::Display, hash::Hash};

#[cfg(feature = "alloc")]
use alloc::borrow::Cow;

/// A time zone with a name
#[derive(Debug, Clone)]
pub struct NamedTimeZone {
    /// The offset, in minutes, from UTC
    offset: i16,

    /// The name of this time zone
    #[cfg(feature = "alloc")]
    name: Cow<'static, str>,

    /// The name of this time zone
    #[cfg(not(feature = "alloc"))]
    name: &'static str,
}

impl NamedTimeZone {
    /// Creates a new [`NamedTimeZone`]
    pub const fn new(offset: i16, name: &'static str) -> Self {
        NamedTimeZone {
            offset,
            #[cfg(feature = "alloc")]
            name: Cow::Borrowed(name),
            #[cfg(not(feature = "alloc"))]
            name,
        }
    }

    /// Creates a new [`NamedTimeZone`]
    #[cfg(feature = "alloc")]
    pub const fn new_owned(offset: i16, name: alloc::string::String) -> Self {
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
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NamedTimeZone {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.offset.cmp(&other.offset)
    }
}

impl Hash for NamedTimeZone {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.offset.hash(state)
    }
}

impl Display for NamedTimeZone {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.name)
    }
}

impl TimeZone for NamedTimeZone {
    const UTC: Self = NamedTimeZone::new(0, "UTC");

    #[cfg(all(feature = "local", feature = "alloc"))]
    fn local() -> Self {
        let (offset, name) = super::get_local_time_zone();

        NamedTimeZone::new_owned(offset, name)
    }

    #[cfg(all(feature = "local", not(feature = "alloc")))]
    fn local() -> Self {
        let offset = super::get_local_time_zone();

        NamedTimeZone::new(offset, "")
    }

    fn offset(&self) -> i16 {
        self.offset
    }
}
