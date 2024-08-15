use core::ptr::null_mut;
use linux::sys::time::timeval;

/// Gets the current time as nanoseconds from the Unix epoch
#[cfg(feature = "now")]
pub(super) fn get_system_time() -> i128 {
    let mut tv = timeval { sec: 0, usec: 0 };
    unsafe { linux::sys::time::gettimeofday(&mut tv, null_mut()) };

    (tv.sec as i128 * 1_000_000_000) + (tv.usec as i128 * 1_000)
}
