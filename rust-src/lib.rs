#![deny(missing_docs)]

//! libmtp
//! ------
//!
//! This is a port of the LibMTP librarry to Rust.

extern crate libc;
extern crate libmtp_sys;

/// The library context
///
/// # Notes
/// You can set the debug level at startup setting the `LIBMTP_DEBUG`
/// environment variable.
#[derive(Debug)]
pub struct Context(*mut libc::c_void);

impl Context {
    /// Creates a new `Context`
    pub fn new() -> Context {
        use std::ptr;

        unsafe { libmtp_sys::LIBMTP_Init() };

        Context(ptr::null_mut())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn context_new() {
        use super::Context;
        use std::ptr;

        let ctx = Context::new();
        assert_eq!(ctx.0, ptr::null_mut());
    }
}
