use core::num::NonZeroI32;

/// Is `year` as leap year?
pub const fn is_leap_year(year: NonZeroI32) -> bool {
    if year.get() % 4 != 0 {
        return false;
    }

    if year.get() % 100 != 0 {
        return true;
    }

    year.get() % 400 == 0
}
