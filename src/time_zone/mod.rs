use core::{
    fmt::{Debug, Display},
    hash::Hash,
};

mod iso8601;
mod named;
mod no;
mod simple;

pub use iso8601::ISO8601TimeZoneDisplay;
pub use named::NamedTimeZone;
pub use no::NoTimeZone;
pub use simple::SimpleTimeZone;

/// A time zone is an offset, in minutes, from a standard time zone: Universal Coordinated Time
pub trait TimeZone:
    Sized + Clone + Debug + Display + PartialEq + Eq + PartialOrd + Ord + Hash
{
    /// The representation of Universal Coordinated Time
    const UTC: Self;

    /// Attempt to get the local time zone
    #[cfg(feature = "local")]
    fn local() -> Option<Self>;

    /// The offset, in minutes, from UTC of this time zone
    fn offset(&self) -> i16;

    /// Creates an [`ISO8601TimeZoneDisplay`] that displays this time zone in ISO 8601 format
    fn iso8601(&self) -> ISO8601TimeZoneDisplay {
        ISO8601TimeZoneDisplay::new(self)
    }
}
