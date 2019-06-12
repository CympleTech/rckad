#[cfg(test)]
mod tests {
    use rckad::KadTree;

    #[test]
    fn it_works() {
        let mut kad = KadTree::new(1, "a");
        kad.add(2, "b");
        kad.add(3, "c");
        kad.add(4, "e");

        assert_eq!(Some((&"b", true)), kad.search(&2));
        assert_eq!(true, kad.contains(&2));

        kad.remove(&2);
        assert_eq!(false, kad.contains(&2));
    }
}
