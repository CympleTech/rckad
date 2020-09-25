use alloc::vec::Vec;
use bit_vec::BitVec;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub struct Distance(BitVec);

impl Distance {
    pub fn max() -> Self {
        Distance(BitVec::from_elem(160, true))
    }

    pub fn min() -> Self {
        Distance(BitVec::from_elem(160, false))
    }

    pub fn new<K: serde::Serialize>(base: &K, target: &K) -> Self {
        let base_byte = postcard::to_allocvec(base).unwrap_or(Vec::new());
        let target_byte = postcard::to_allocvec(target).unwrap_or(Vec::new());

        let hash1 = blake3::hash(&base_byte);
        let base_source = BitVec::from_bytes(hash1.as_bytes());
        let base = Distance((0..160).map(|i| base_source[i]).collect());

        let hash2 = blake3::hash(&target_byte);
        let target_source = BitVec::from_bytes(hash2.as_bytes());
        let target = Distance((0..160).map(|i| target_source[i]).collect());

        base.xor(&target)
    }

    pub fn get(&self, index: usize) -> bool {
        if index >= 160 {
            false
        } else {
            self.0[index]
        }
    }

    pub fn xor(&self, other: &Distance) -> Distance {
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
        Distance::min()
    }
}
