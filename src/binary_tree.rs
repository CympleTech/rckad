use serde::Serialize;
use std::cmp::Ordering;

use crate::distance::Distance;

const MAX_LEVEL: usize = 8;

pub struct KadTree<K: PartialEq + Serialize, V> {
    root_key: K,
    left: TreeNode<K, V>,
    right: TreeNode<K, V>,
    k_bucket: usize,
}

type TreeNode<K, V> = Option<Box<Node<K, V>>>;

pub struct Node<K: PartialEq + Serialize, V> {
    left: TreeNode<K, V>,
    right: TreeNode<K, V>,
    list: Vec<Cell<K, V>>,
}

struct Cell<K: PartialEq, V>(K, V, Distance);

impl<K: PartialEq + Serialize, V> KadTree<K, V> {
    pub fn new(key: K) -> Self {
        KadTree {
            root_key: key,
            left: None,
            right: None,
            k_bucket: 8, // Default K_BUCKET
        }
    }

    pub fn with_k_bucket(key: K, bucket: usize) -> Self {
        KadTree {
            root_key: key,
            left: None,
            right: None,
            k_bucket: bucket,
        }
    }

    pub fn add(&mut self, key: K, value: V) -> bool {
        let distance = Distance::new::<K>(&self.root_key, &key);
        let k_bucket = self.k_bucket.clone();

        if distance.get(0) {
            if self.right.is_none() {
                self.right = Some(Box::new(Node::default()));
            }
            self.right
                .as_mut()
                .and_then(|v| Some(v.insert(Cell(key, value, distance), 1, k_bucket)))
                .unwrap()
        } else {
            if self.left.is_none() {
                self.left = Some(Box::new(Node::default()));
            }
            self.left
                .as_mut()
                .and_then(|v| Some(v.insert(Cell(key, value, distance), 1, k_bucket)))
                .unwrap()
        }
    }

    pub fn search(&self, key: &K) -> Option<(&K, &V, bool)> {
        let distance = Distance::new::<K>(&self.root_key, &key);
        if distance.get(0) {
            if self.right.is_none() {
                return None;
            };

            self.right
                .as_ref()
                .and_then(|v| Some(v.search(key, &distance, 1)))
                .unwrap()
        } else {
            if self.left.is_none() {
                return None;
            };

            self.left
                .as_ref()
                .and_then(|v| Some(v.search(key, &distance, 1)))
                .unwrap()
        }
    }

    pub fn remove(&mut self, key: &K) {
        let distance = Distance::new::<K>(&self.root_key, &key);
        if distance.get(0) {
            self.right
                .as_mut()
                .and_then(|v| Some(v.remove(key, &distance, 1)));
        } else {
            self.left
                .as_mut()
                .and_then(|v| Some(v.remove(key, &distance, 1)));
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        if let Some((_, _, true)) = self.search(key) {
            true
        } else {
            false
        }
    }
}

impl<K: PartialEq + Serialize, V> Node<K, V> {
    fn default() -> Self {
        Node {
            left: None,
            right: None,
            list: vec![],
        }
    }

    fn insert(&mut self, mut cell: Cell<K, V>, index: usize, k_bucket: usize) -> bool {
        if self.right.is_some() || self.left.is_some() {
            if cell.2.get(index) {
                if self.right.is_none() {
                    self.right = Some(Box::new(Node::default()));
                }
                self.right
                    .as_mut()
                    .and_then(|v| Some(v.insert(cell, index + 1, k_bucket)))
                    .unwrap()
            } else {
                if self.left.is_none() {
                    self.left = Some(Box::new(Node::default()));
                }
                self.left
                    .as_mut()
                    .and_then(|v| Some(v.insert(cell, index + 1, k_bucket)))
                    .unwrap()
            }
        } else {
            let mut need_deleted = std::usize::MAX;
            for (i, c) in self.list.iter().enumerate() {
                if c == &cell {
                    need_deleted = i;
                }
            }
            if need_deleted != std::usize::MAX {
                self.list.remove(need_deleted);
            }

            if self.list.len() < k_bucket {
                self.list.push(cell);
                true
            } else {
                if index >= MAX_LEVEL {
                    for v in self.list.iter_mut() {
                        if v > &mut cell {
                            *v = cell;
                            return true;
                        }
                    }
                    return false;
                } else {
                    self.right = Some(Box::new(Node::default()));
                    self.left = Some(Box::new(Node::default()));

                    while !self.list.is_empty() {
                        let new_cell = self.list.remove(0);
                        self.insert(new_cell, index, k_bucket);
                    }

                    self.insert(cell, index, k_bucket)
                }
            }
        }
    }

    pub fn search(&self, key: &K, distance: &Distance, index: usize) -> Option<(&K, &V, bool)> {
        let mut closest_index = std::usize::MAX;
        let mut closest_distance = Distance::max();

        for (index, cell) in self.list.iter().enumerate() {
            if &cell.0 == key {
                return Some((&cell.0, &cell.1, true));
            } else {
                let dis = distance.xor(&cell.2);
                if dis < closest_distance {
                    closest_distance = dis;
                    closest_index = index;
                }
            }
        }

        if distance.get(index) {
            if let Some(ref right) = self.right {
                let next = right.search(key, distance, index + 1);
                if next.is_some() {
                    return next;
                }
            }
        } else {
            if let Some(ref left) = self.left {
                let next = left.search(key, distance, index + 1);
                if next.is_some() {
                    return next;
                }
            }
        }

        self.list
            .get(closest_index)
            .and_then(|cell| Some((&cell.0, &cell.1, false)))
    }

    pub fn remove(&mut self, key: &K, distance: &Distance, index: usize) {
        let mut deleted_index = std::usize::MAX;
        for (i, cell) in self.list.iter().enumerate() {
            if &cell.0 == key {
                deleted_index = i;
            }
        }

        if deleted_index != std::usize::MAX {
            self.list.remove(deleted_index);
            return;
        }

        if distance.get(index) {
            if let Some(ref mut right) = self.right {
                right.remove(key, distance, index + 1);
            }
        } else {
            if let Some(ref mut left) = self.left {
                left.remove(key, distance, index + 1);
            }
        }
    }
}

impl<K: PartialEq, V> Ord for Cell<K, V> {
    fn cmp(&self, other: &Cell<K, V>) -> Ordering {
        self.2.cmp(&other.2)
    }
}

impl<K: PartialEq, V> PartialOrd for Cell<K, V> {
    fn partial_cmp(&self, other: &Cell<K, V>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: PartialEq, V> Eq for Cell<K, V> {}

impl<K: PartialEq, V> PartialEq for Cell<K, V> {
    fn eq(&self, other: &Cell<K, V>) -> bool {
        self.0 == other.0
    }
}
