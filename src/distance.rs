use bit_vec::BitVec;
use serde::Serialize;
use sha3::{Digest, Sha3_256};

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Distance(BitVec);

impl Distance {
    pub fn new<K: Serialize>(base: &K, target: &K) -> Self {
        let base_byte = bincode::serialize(base).unwrap();
        let target_byte = bincode::serialize(target).unwrap();

        let mut base_hasher = Sha3_256::new();
        base_hasher.input(base_byte);
        let base_source = BitVec::from_bytes(&base_hasher.result());
        let base = Distance((0..160).map(|i| base_source[i]).collect());

        let mut target_hasher = Sha3_256::new();
        target_hasher.input(target_byte);
        let target_source = BitVec::from_bytes(&target_hasher.result());
        let target = Distance((0..160).map(|i| target_source[i]).collect());

        base.xor(&target)
    }

    fn xor(&self, other: &Distance) -> Distance {
        let mut new_binary = BitVec::from_elem(160, false);

        for i in 0..160 {
            if self.0[i] != other.0[i] {
                new_binary.set(i, true);
            } else {
                new_binary.set(i, false);
            }
        }

        Distance(new_binary)
    }
}

impl Default for Distance {
    fn default() -> Self {
        Distance(BitVec::with_capacity(160))
    }
}
