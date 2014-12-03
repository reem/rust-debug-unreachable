#![deny(missing_docs, warnings)]
#![feature(macro_rules)]

//! `unreachable!()` in debug builds, `intrinsics::unreachable` in release.

#[macro_export]
/// `unreachable!()` in debug builds, `intrinsics::unreachable` in release.
macro_rules! debug_unreachable {
    () => {
        if cfg!(ndebug) {
            ::std::intrinsics::unreachable()
        } else {
            unreachable!()
        }
    }
}

