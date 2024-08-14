use core::num::{NonZeroI32, NonZeroU8};

use crate::month_length;

/// The values used for the calculation
///
/// Derived from `floor(2.6(m + 1) - 0.2)` where `m` is the index of the month, starting with 0 as
/// March and 12 as February
const MONTH_VALUE: [usize; 12] = [2, 5, 7, 10, 12, 15, 18, 20, 23, 25, 28, 31];

/// Gets the day of the week for the given `day`, `month`, and `year`
///
/// The day of the week is 0-indexed, where 0 is Sunday and 6 is Saturday
pub const fn day_of_week(year: NonZeroI32, month: u8, day: NonZeroU8) -> u8 {
    assert!(month < 12);
    assert!(day.get() <= month_length(year, month).get());

    let positive = year.is_positive();
    let year = year.abs();

    let k = day.get() as isize;
    let m = MONTH_VALUE[if month < 2 { month + 10 } else { month - 2 } as usize] as isize;
    let c = (year.get() / 100) as isize;
    let y = (year.get() % 100) as isize;

    ((k + m + y + (y / 4) + (c / 4) - (2 * c)) % 7).abs() as u8
}
