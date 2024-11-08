#![no_std]

#![feature(type_alias_impl_trait)]
#![feature(try_trait_v2)]

pub mod oneshot;

/// This exports the internal dependencies.
pub mod x_deps {
    pub use mm_ptr;
    pub use pincol;

    pub use mm_ptr::x_deps::atomic_sync;
    pub use atomic_sync::x_deps::{abs_sync, atomex};
    pub use abs_sync::x_deps::pin_utils;
}