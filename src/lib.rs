#[cfg(feature="bindgen")]
mod ffi;
#[cfg(not(feature="bindgen"))]
mod pbsffi;
#[cfg(not(feature="bindgen"))]
use pbsffi as ffi;

pub use ffi::*;
