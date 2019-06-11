use std::cmp::Ordering;

type Binary = [bool; 160];

pub(crate) struct Distance(Binary);

impl Distance {
    pub fn new<K>(_base: &K, _target: &K) -> Self {
        Distance::default()
    }

    fn distance(&self) -> &Binary {
        &self.0
    }
}

impl Default for Distance {
    fn default() -> Self {
        Distance([false; 160])
    }
}

impl Eq for Distance {}

impl Ord for Distance {
    fn cmp(&self, other: &Distance) -> Ordering {
        for i in 0..160 {
            if self.distance()[i] != other.distance()[i] {
                return self.distance()[i].cmp(&other.distance()[i]);
            }
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Distance) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Distance {
    fn eq(&self, other: &Distance) -> bool {
        for i in 0..160 {
            if self.distance()[i] != other.distance()[i] {
                return false;
            }
        }
        return true;
    }
}
