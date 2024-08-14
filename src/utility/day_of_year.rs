use crate::{is_leap_year, month_length};
use core::num::{NonZeroI32, NonZeroU8};

/// The cumulative days passed in the year on the first of each month
const MONTH_CULMULATIVE: [u16; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

/// Gets the day of the year for `day`, `month` and `year`
pub const fn day_of_year(year: NonZeroI32, month: u8, day: NonZeroU8) -> u16 {
    assert!(month < 12);
    assert!(day.get() < month_length(year, month).get());

    MONTH_CULMULATIVE[month as usize] + day.get() as u16 - 1
        + if month > 2 && is_leap_year(year) {
            1
        } else {
            0
        }
}
