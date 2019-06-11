use std::cmp::Ordering;

type TreeNode<K, V> = Option<Box<Node<K, V>>>;

pub type KadTree<K, V> = Node<K, V>;

use crate::distance::Distance;

pub struct Node<K: PartialEq, V> {
    left: TreeNode<K, V>,
    right: TreeNode<K, V>,
    key: K,
    value: V,
    distance: Distance,
}

impl<K: PartialEq, V> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self::root(key, value)
    }

    pub fn root(key: K, value: V) -> Self {
        Node {
            left: None,
            right: None,
            key: key,
            value: value,
            distance: Distance::default(),
        }
    }

    pub fn add(&self, key: K, value: V) -> Self {
        let distance = Distance::new::<K>(&self.key, &key);
        Node {
            left: None,
            right: None,
            key: key,
            value: value,
            distance: distance,
        }
    }

    pub fn insert(&mut self, node: Node<K, V>) -> bool {
        if self.distance < node.distance {
            if let Some(ref mut right) = self.right {
                if right.key == node.key {
                    right.value = node.value;
                } else {
                    return right.insert(node);
                }
            } else {
                self.right = Some(Box::new(node));
            }
        } else {
            if let Some(ref mut left) = self.left {
                if left.key == node.key {
                    left.value = node.value;
                } else {
                    return left.insert(node);
                }
            } else {
                self.left = Some(Box::new(node));
            }
        }
        true
    }

    pub fn search(&self, key: &K) -> Option<(&V, bool)> {
        if &self.key == key {
            return Some((&self.value, true));
        }

        if let Some(ref left) = self.left {
            let next = left.search(key);
            if next.is_some() {
                return next;
            }
        }

        if let Some(ref right) = self.right {
            let next = right.search(key);
            if next.is_some() {
                return next;
            }
        }

        None
    }

    pub fn remove(&mut self, key: &K) {
        if let Some(ref mut left) = self.left {
            if &left.key == key {
                self.left = None;
                return;
            }
            left.remove(key);
        }

        if let Some(ref mut right) = self.right {
            if &right.key == key {
                self.right = None;
                return;
            }

            right.remove(key);
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        if let Some((_, true)) = self.search(key) {
            true
        } else {
            false
        }
    }
}

impl<K: PartialEq, V> Ord for Node<K, V> {
    fn cmp(&self, other: &Node<K, V>) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl<K: PartialEq, V> PartialOrd for Node<K, V> {
    fn partial_cmp(&self, other: &Node<K, V>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: PartialEq, V> Eq for Node<K, V> {}

impl<K: PartialEq, V> PartialEq for Node<K, V> {
    fn eq(&self, other: &Node<K, V>) -> bool {
        self.key == other.key
    }
}
