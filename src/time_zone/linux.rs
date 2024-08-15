use std::sync::Once;

static TZ_SET: Once = Once::new();

#[cfg(all(feature = "alloc", feature = "local"))]
pub(super) fn get_local_time_zone() -> (i16, alloc::string::String) {
    tzset_once();

    let offset = -unsafe { linux::time::timezone } / 60;
    let name = if unsafe { linux::time::tzname[0] } == core::ptr::null() {
        alloc::string::String::new()
    } else {
        unsafe { std::ffi::CStr::from_ptr(linux::time::tzname[0]) }
            .to_string_lossy()
            .to_string()
    };

    (offset as _, name)
}

#[cfg(all(not(feature = "alloc"), feature = "local"))]
pub(super) fn get_local_time_zone() -> i16 {
    tzset_once();

    (-unsafe { linux::time::timezone } / 60) as _
}

fn tzset_once() {
    TZ_SET.call_once(|| unsafe { linux::time::tzset() });
}
