//! Probe if a device is a MTP device.

use libmtp_sys;
use libc::c_int;

use ::Context;

// TODO: Port LIBMTP_Check_Specific_Device to rust.

/// Checks if the given device is a MTP device.
pub fn check_device(_context: &Context, bus_number: u8, address: u8) -> bool {
    let ret =
        unsafe { libmtp_sys::LIBMTP_Check_Specific_Device(bus_number as c_int, address as c_int) };
    ret == 1
}
