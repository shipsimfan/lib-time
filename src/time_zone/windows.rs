use win32::TIME_ZONE_INFORMATION;
use win32::{TIME_ZONE_ID_DAYLIGHT, TIME_ZONE_ID_STANDARD, TIME_ZONE_ID_UNKNOWN};

#[cfg(all(feature = "alloc", feature = "local"))]
pub(super) fn get_local_time_zone() -> (i16, alloc::string::String) {
    let mut time_zone_info = TIME_ZONE_INFORMATION::default();
    let result = unsafe { win32::GetTimeZoneInformation(&mut time_zone_info) };

    let daylight = match result {
        TIME_ZONE_ID_STANDARD | TIME_ZONE_ID_UNKNOWN => false,
        TIME_ZONE_ID_DAYLIGHT => true,
        _ => return (0, alloc::string::String::new()),
    };

    let (bias, name_utf16) = if daylight {
        (
            time_zone_info.bias + time_zone_info.daylight_bias,
            &time_zone_info.daylight_name,
        )
    } else {
        (
            time_zone_info.bias + time_zone_info.standard_bias,
            &time_zone_info.standard_name,
        )
    };

    let mut name_len = name_utf16.len();
    for i in 0..name_utf16.len() {
        if name_utf16[i] == 0 {
            name_len = i;
            break;
        }
    }

    let name = alloc::string::String::from_utf16_lossy(&name_utf16[..name_len]);

    (-(bias as i16), name)
}

#[cfg(all(not(feature = "alloc"), feature = "local"))]
pub(super) fn get_local_time_zone() -> i16 {
    let mut time_zone_info = TIME_ZONE_INFORMATION::default();
    let result = unsafe { win32::GetTimeZoneInformation(&mut time_zone_info) };

    let daylight = match result {
        TIME_ZONE_ID_STANDARD | TIME_ZONE_ID_UNKNOWN => false,
        TIME_ZONE_ID_DAYLIGHT => true,
        _ => return (0, alloc::string::String::new()),
    };

    -(time_zone_info.bias
        + if daylight {
            time_zone_info.daylight_bias
        } else {
            time_zone_info.standard_bias
        }) as i16
}
