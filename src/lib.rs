//! Utilities for handling dates, times, and time zones

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod time_zone;
mod timestamp;

pub use time_zone::{ISO8601TimeZoneDisplay, NamedTimeZone, NoTimeZone, SimpleTimeZone, TimeZone};
pub use timestamp::Timestamp;
