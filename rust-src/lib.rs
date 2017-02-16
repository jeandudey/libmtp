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
pub struct Context(libmtp_sys::LIBMTP_Context);

impl Context {
    /// Creates a new `Context`
    pub fn new() -> Context {
        let context = unsafe { libmtp_sys::LIBMTP_Init() };

        Context(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_new() {
        let ctx = Context::new();
        assert!(ctx.0.use_mtpz == 0 || ctx.0.use_mtpz == 1);
    }
}
