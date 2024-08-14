use core::num::NonZeroU8;

/// The full names of the months
pub const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// The names of the months shortened to only 3 letters
pub const MONTH_SHORT_NAMES: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// The maximum value the day of the month can take for each month``
pub const MONTH_LENGTHS: [NonZeroU8; 12] = unsafe {
    [
        NonZeroU8::new_unchecked(31),
        NonZeroU8::new_unchecked(29),
        NonZeroU8::new_unchecked(31),
        NonZeroU8::new_unchecked(30),
        NonZeroU8::new_unchecked(31),
        NonZeroU8::new_unchecked(30),
        NonZeroU8::new_unchecked(31),
        NonZeroU8::new_unchecked(31),
        NonZeroU8::new_unchecked(30),
        NonZeroU8::new_unchecked(31),
        NonZeroU8::new_unchecked(30),
        NonZeroU8::new_unchecked(31),
    ]
};

/// The index for January
pub const JANUARY: u8 = 0;

/// The index for February
pub const FEBRUARY: u8 = 1;

/// The index for March
pub const MARCH: u8 = 2;

/// The index for April
pub const APRIL: u8 = 3;

/// The index for May
pub const MAY: u8 = 4;

/// The index for June
pub const JUNE: u8 = 5;

/// The index for July
pub const JULY: u8 = 6;

/// The index for August
pub const AUGUST: u8 = 7;

/// The index for September
pub const SEPTEMBER: u8 = 8;

/// The index for October
pub const OCTOBER: u8 = 9;

/// The index for November
pub const NOVEMBER: u8 = 10;

/// The index for December
pub const DECEMBER: u8 = 11;

/// The full names of the weekdays
pub const WEEKDAY_NAMES: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

/// The names of the weekdays shortened to only 3 letters
pub const WEEKDAY_SHORT_NAMES: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

/// The index for Sunday
pub const SUNDAY: u8 = 0;

/// The index for Monday
pub const MONDAY: u8 = 1;

/// The index for Tuesday
pub const TUESDAY: u8 = 2;

/// The index for Wednesday
pub const WEDNESDAY: u8 = 3;

/// The index for Thursday
pub const THURSDAY: u8 = 4;

/// The index for Friday
pub const FRIDAY: u8 = 5;

/// The index for Saturday
pub const SATURDAY: u8 = 6;
