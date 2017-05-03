use std::hash::{Hasher, BuildHasher};
use std::mem::transmute;

pub fn f64_as_u64(f: f64) -> u64 {
    // FIXME: if this functionality is added to the standard library then use it instead.
    // this is safe because f64 and u64 are of the same size and any possible u64 is valid.
    unsafe { transmute::<f64, u64>(f) }
}

pub struct IdIdentityHasher(u64);

#[derive(Copy, Clone, Default)]
pub struct IdIdentityHasherBuilder;

impl Hasher for IdIdentityHasher {
    fn write_u64(&mut self, i: u64) {
        self.0 ^= i;
    }

    fn finish(&self) -> u64 {
        debug_assert_ne!(self.0, 0);

        self.0
    }

    fn write(&mut self, _bytes: &[u8]) {
        panic!("IdIdentityHasher should only be used for hashing identifiers");
    }
}

impl BuildHasher for IdIdentityHasherBuilder {
    type Hasher = IdIdentityHasher;

    fn build_hasher(&self) -> Self::Hasher {
        IdIdentityHasher(0)
    }
}
