#[cfg(feature = "now")]
pub(super) fn get_current_date_time_utc<T: crate::TimeZone>() -> crate::DateTime<T> {
    let time = unsafe { linux::time::time(core::ptr::null_mut()) };
    let gmtime = unsafe { linux::time::gmtime(&time) };
    let gmtime = if gmtime == core::ptr::null_mut() {
        return crate::DateTime::new(
            core::num::NonZeroI32::new(1970).unwrap(),
            0,
            core::num::NonZeroU8::new(1).unwrap(),
            0,
            0,
            0,
            0,
            0,
            0,
            T::local(),
        );
    } else {
        unsafe { &*gmtime }
    };

    crate::DateTime::new(
        core::num::NonZeroI32::new(gmtime.year + 1900).unwrap(),
        gmtime.mon as _,
        core::num::NonZeroU8::new(gmtime.mday as _).unwrap(),
        gmtime.hour as _,
        gmtime.min as _,
        gmtime.sec as _,
        0,
        0,
        0,
        T::UTC,
    )
}

#[cfg(all(feature = "now", feature = "local"))]
pub(super) fn get_current_date_time_local<T: crate::TimeZone>() -> crate::DateTime<T> {
    let time = unsafe { linux::time::time(core::ptr::null_mut()) };
    let localtime = unsafe { linux::time::localtime(&time) };
    let localtime = if localtime == core::ptr::null_mut() {
        return crate::DateTime::new(
            core::num::NonZeroI32::new(1970).unwrap(),
            0,
            core::num::NonZeroU8::new(1).unwrap(),
            0,
            0,
            0,
            0,
            0,
            0,
            T::local(),
        );
    } else {
        unsafe { &*localtime }
    };

    crate::DateTime::new(
        core::num::NonZeroI32::new(localtime.year + 1900).unwrap(),
        localtime.mon as _,
        core::num::NonZeroU8::new(localtime.mday as _).unwrap(),
        localtime.hour as _,
        localtime.min as _,
        localtime.sec as _,
        0,
        0,
        0,
        T::local(),
    )
}
