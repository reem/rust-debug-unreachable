#![deny(missing_docs, warnings)]

//! `panic!()` in debug builds, `intrinsics::unreachable` in release.

#[macro_export]
/// `panic!()` in debug builds, `intrinsics::unreachable` in release.
macro_rules! debug_unreachable {
    () => { debug_unreachable!("entered unreachable code") };
    ($e:expr) => {
        if cfg!(ndebug) {
            ::std::intrinsics::unreachable();
        } else {
            panic!($e);
        }
    }
}

