//! Utilities for handling dates, times, and time zones

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod time_zone;

pub use time_zone::{ISO8601TimeZoneDisplay, NamedTimeZone, NoTimeZone, SimpleTimeZone, TimeZone};
