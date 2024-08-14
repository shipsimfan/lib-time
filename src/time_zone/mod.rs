use core::{
    fmt::{Debug, Display},
    hash::Hash,
};

mod iso8601;
mod named;
mod no;
mod simple;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(all(target_os = "linux"))]
mod linux;

pub use iso8601::TimeZoneISO8601Display;
pub use named::NamedTimeZone;
pub use no::NoTimeZone;
pub use simple::SimpleTimeZone;

#[cfg(all(target_os = "windows", feature = "local"))]
use windows::get_local_time_zone;

#[cfg(all(target_os = "linux", feature = "local"))]
use linux::get_local_time_zone;

/// A time zone is an offset, in minutes, from a standard time zone: Universal Coordinated Time
pub trait TimeZone:
    Sized + Clone + Debug + Display + PartialEq + Eq + PartialOrd + Ord + Hash
{
    /// The representation of Universal Coordinated Time
    const UTC: Self;

    /// Attempt to get the local time zone
    #[cfg(feature = "local")]
    fn local() -> Self;

    /// The offset, in minutes, from UTC of this time zone
    fn offset(&self) -> i16;

    /// Creates an [`ISO8601TimeZoneDisplay`] that displays this time zone in ISO 8601 format
    fn iso8601(&self) -> TimeZoneISO8601Display {
        TimeZoneISO8601Display::new(self)
    }
}
