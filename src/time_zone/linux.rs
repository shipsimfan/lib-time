#[cfg(all(feature = "alloc", feature = "local"))]
pub(super) fn get_local_time_zone() -> (i16, alloc::string::String) {
    let time = unsafe { linux::time::time(core::ptr::null_mut()) };
    let localtime = unsafe { linux::time::localtime(&time) };
    let localtime = if localtime == core::ptr::null_mut() {
        return (0, alloc::string::String::new());
    } else {
        unsafe { &*localtime }
    };

    let offset = localtime.gmtoff / 60;
    let name = if localtime.zone == core::ptr::null() {
        alloc::string::String::new()
    } else {
        unsafe { std::ffi::CStr::from_ptr(localtime.zone) }
            .to_string_lossy()
            .to_string()
    };

    (offset as _, name)
}

#[cfg(all(not(feature = "alloc"), feature = "local"))]
pub(super) fn get_local_time_zone() -> i16 {
    let time = unsafe { linux::time::time(core::ptr::null_mut()) };
    let localtime = unsafe { linux::time::localtime(&time) };
    let localtime = if localtime == core::ptr::null_mut() {
        return (0, alloc::string::String::new());
    } else {
        unsafe { &*localtime }
    };

    localtime.gmtoff / 60
}
