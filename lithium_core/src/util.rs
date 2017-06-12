use std::hash::{Hasher, BuildHasher};
use std::mem::transmute;

pub fn f64_as_u64(f: f64) -> u64 {
    // FIXME: if this functionality is added to the standard library then use it instead.
    //
    // This is safe because f64 and u64 are of the same size and any possible u64 is valid.
    unsafe { transmute::<f64, u64>(f) }
}

/// Hasher which can be used with identifiers.
///
/// To speed up `HashMap`s we are using custom hasher. We can avoid costly hashing algorithms because all identifiers
/// used inside of this library are either randomly generated or generated using sufficiently good hashes.
pub struct IdIdentityHasher(u64);

/// Helper type which is necessary to use `IdIdentityHasher` with `HashMap`.
#[derive(Copy, Clone, Default)]
pub struct IdIdentityHasherBuilder;

impl Hasher for IdIdentityHasher {
    // Identifiers always consist of a bunch of `u64`s so we just xor them together.
    fn write_u64(&mut self, i: u64) {
        self.0 ^= i;
    }

    fn finish(&self) -> u64 {
        // Ensure that something was actually hashed before finishing.
        // Hitting this assertion by accident is improbable and it is enabled only in debug builds anyway.
        debug_assert_ne!(self.0, 0);

        self.0
    }

    fn write(&mut self, _bytes: &[u8]) {
        // Detect attempts to use this hasher with inapropriate types.
        panic!("IdIdentityHasher should only be used for hashing identifiers");
    }
}

impl BuildHasher for IdIdentityHasherBuilder {
    type Hasher = IdIdentityHasher;

    fn build_hasher(&self) -> Self::Hasher {
        IdIdentityHasher(0)
    }
}
