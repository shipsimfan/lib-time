//! Utilities for handling dates, times, and time zones

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod constants;
mod date_time;
mod time_zone;
mod timestamp;
mod utility;

pub use constants::*;
pub use date_time::{DateTime, DateTimeFullDisplay, DateTimeISO8601Display, DateTimeShortDisplay};
pub use time_zone::{NamedTimeZone, NoTimeZone, SimpleTimeZone, TimeZone, TimeZoneISO8601Display};
pub use timestamp::Timestamp;
pub use utility::{day_of_week, day_of_year, is_leap_year, month_length, week_of_year};
