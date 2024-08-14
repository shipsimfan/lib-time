use super::is_leap_year;
use crate::MONTH_LENGTHS;
use core::num::{NonZeroI32, NonZeroU8};

/// Gets the length of `month` in `year`
pub const fn month_length(year: NonZeroI32, month: u8) -> NonZeroU8 {
    assert!(month < 12);

    if month != 1 {
        return MONTH_LENGTHS[month as usize];
    }

    unsafe { NonZeroU8::new_unchecked(if is_leap_year(year) { 29 } else { 28 }) }
}
