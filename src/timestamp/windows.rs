use win32::{GetSystemTimeAsFileTime, FILETIME};

/// The Unix epoch in 100 nanoseconds from the Windows epoch
#[cfg(feature = "now")]
const EPOCH_OFFSET: i128 = 116444736000000000;

/// The conversion from [`FILETIME`] to nanoseconds
#[cfg(feature = "now")]
const CONVERSION: i128 = 100;

/// Gets the current time as nanoseconds from the Unix epoch
#[cfg(feature = "now")]
pub(super) fn get_system_time() -> i128 {
    let mut file_time = FILETIME {
        low_date_time: 0,
        high_date_time: 0,
    };
    unsafe { GetSystemTimeAsFileTime(&mut file_time) };

    ((file_time.low_date_time as i128 | ((file_time.high_date_time as i128) << 32)) - EPOCH_OFFSET)
        * CONVERSION
}
