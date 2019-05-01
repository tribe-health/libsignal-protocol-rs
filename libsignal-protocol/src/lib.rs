#[macro_use]
mod macros;
mod context;
mod crypto;
mod errors;
mod keys;

pub use crate::context::Context;
pub use crate::crypto::{Crypto, DefaultCrypto};
pub use crate::errors::InternalError;
pub use crate::keys::{IdentityKeyPair, PreKeyList};

pub(crate) trait Wrapped: Sized {
    type Raw: ?Sized;

    unsafe fn from_raw(raw: *mut Self::Raw) -> Self;
    fn raw(&self) -> *const Self::Raw;
    fn raw_mut(&mut self) -> *mut Self::Raw;
}