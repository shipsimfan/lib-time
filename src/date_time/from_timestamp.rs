use core::num::{NonZeroI32, NonZeroU8};
use std::time::SystemTime;

use crate::{DateTime, TimeZone, Timestamp};

impl<T: TimeZone> From<Timestamp<T>> for DateTime<T> {
    fn from(timestamp: Timestamp<T>) -> Self {
        let positive = timestamp.as_nanos().is_positive();
        let total_nanoseconds = timestamp.as_nanos().abs() as u128;

        let nanosecond = (total_nanoseconds % 1_000) as u16;
        let total_microseconds = total_nanoseconds / 1_000;

        let microsecond = (total_microseconds % 1_000) as u16;
        let total_milliseconds = total_microseconds / 1_000;

        let millisecond = (total_milliseconds % 1_000) as u16;
        let total_seconds = (total_milliseconds / 1_000) as u64;

        let second = (total_seconds % 60) as u8;
        let total_minutes = total_seconds / 60;

        let minute = (total_minutes % 60) as u8;
        let total_hours = total_minutes / 60;

        let hour = (total_hours % 24) as u8;
        let total_days = total_hours / 24;

        let days_since_0 = total_days + 719468;
        let era = days_since_0 / 146097;
        let doe = days_since_0 - (era * 146097);
        let yoe = (doe - (doe / 1460) + (doe / 36524) - (doe / 146096)) / 365;
        let year = yoe + era * 400;
        let doy = doe - (365 * yoe + (yoe / 4) - (yoe / 400));
        let mp = ((5 * doy) + 2) / 153;
        let day = NonZeroU8::new((doy - ((153 * mp) + 2) / 5 + 1) as u8).unwrap();
        let month = if mp < 10 { mp + 3 } else { mp - 9 } as u8 - 1;

        let year = NonZeroI32::new(year as i32 * if positive { 1 } else { -1 }).unwrap();

        DateTime::new(
            year,
            month,
            day,
            hour,
            minute,
            second,
            millisecond,
            microsecond,
            nanosecond,
            timestamp.take_timezone(),
        )
    }
}

impl<T: TimeZone> From<SystemTime> for DateTime<T> {
    fn from(system_time: SystemTime) -> Self {
        Timestamp::<T>::from(system_time).into()
    }
}

#[test]
fn testing() {
    let mut date_time: DateTime = Timestamp::now_local().into();
    date_time.set_year(NonZeroI32::new(1506).unwrap());

    println!("{}", date_time.full_display());
}
