#[cfg(test)]
mod tests {
    use rckad::KadTree;

    #[test]
    fn it_works() {
        let mut kad = KadTree::new(0, "0");
        kad.add(2, "b");
        kad.add(3, "c");
        kad.add(4, "e");

        assert_eq!(Some((&2, &"b", true)), kad.search(&2));
        assert_eq!(true, kad.contains(&2));

        kad.remove(&2);
        assert_eq!(false, kad.contains(&2));

        let mut kad = KadTree::with_k_bucket(0, "0".to_owned(), 2);

        for i in 1..(256 * 2 + 2) {
            kad.add(i, format!("{}", i));
        }

        assert_eq!(Some((&61, &"61".to_owned(), false)), kad.search(&131));
    }
}
