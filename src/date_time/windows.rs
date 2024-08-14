#[cfg(feature = "now")]
pub(super) fn get_current_date_time_utc<T: crate::TimeZone>() -> crate::DateTime<T> {
    let mut platform_time = win32::SYSTEMTIME::default();
    unsafe { win32::GetSystemTime(&mut platform_time) };
    create_date_time(&platform_time, T::UTC)
}

#[cfg(all(feature = "now", feature = "local"))]
pub(super) fn get_current_date_time_local<T: crate::TimeZone>() -> crate::DateTime<T> {
    let mut platform_time = win32::SYSTEMTIME::default();
    unsafe { win32::GetLocalTime(&mut platform_time) };
    create_date_time(&platform_time, T::local())
}

#[cfg(feature = "now")]
fn create_date_time<T: crate::TimeZone>(
    platform_time: &win32::SYSTEMTIME,
    time_zone: T,
) -> crate::DateTime<T> {
    crate::DateTime::new(
        std::num::NonZeroI32::new(platform_time.year as _).unwrap(),
        platform_time.month as u8 - 1,
        std::num::NonZeroU8::new(platform_time.day as _).unwrap(),
        platform_time.hour as _,
        platform_time.minute as _,
        platform_time.second as _,
        platform_time.milliseconds as _,
        0,
        0,
        time_zone,
    )
}
