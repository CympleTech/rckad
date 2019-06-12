#[cfg(test)]
mod tests {
    use rckad::KadTree;

    #[test]
    fn it_works() {
        let kad = KadTree::new(1, "a");
        kad.add(2, "b");
        kad.add(3, "c");
        kad.add(4, "e");

        assert_eq!(2 + 2, 4);
    }
}
