use rand::{thread_rng, Rng};
use {Rect, Var};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub struct Id(u64, u64);

impl Id {
    /// Generate new unique id.
    pub fn unique() -> Self {
        Id(thread_rng().next_u64(), thread_rng().next_u64())
    }
}

impl From<Id> for Rect<Var> {
    fn from(id: Id) -> Self {
        Rect {
            left:   Var::from(Id(hash_combine(id.0, 0x704ddeddddcd8305), hash_combine(id.1, 0x9d708bad4e0dfa92))),
            right:  Var::from(Id(hash_combine(id.1, 0x783da78e81729554), hash_combine(id.0, 0xd93e20a6528175cb))),
            top:    Var::from(Id(hash_combine(id.0, 0x8c4dd4132e69c870), hash_combine(id.1, 0xa2738decbe5a0105))),
            bottom: Var::from(Id(hash_combine(id.1, 0x92b0c372db71a2d4), hash_combine(id.0, 0x66f27a9be0b03507))),
        }
    }
}

fn hash_combine(a: u64, b: u64) -> u64 {
    // is this good?
    a ^ (b.wrapping_add(a.rotate_left(17)).wrapping_add(a >> 2))
}