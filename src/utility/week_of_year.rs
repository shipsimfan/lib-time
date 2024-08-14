use super::day_of_year;
use core::num::{NonZeroI32, NonZeroU8};

/// Gets the week of the year
pub const fn week_of_year(year: NonZeroI32, month: u8, day: NonZeroU8) -> u8 {
    (day_of_year(year, month, day) / 7) as u8
}
