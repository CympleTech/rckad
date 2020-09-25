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

        assert_eq!(Some("b"), kad.remove(&2));
        assert_eq!(None, kad.remove(&5));

        assert_eq!(false, kad.contains(&2));

        let mut kad = KadTree::with_k_bucket(0, "0".to_owned(), 2);

        for i in 1..(256 * 2 + 2) {
            kad.add(i, format!("{}", i));
        }

        for i in kad.keys() {
            println!("key: {}", i);
        }

        println!("0 {:?}", blake3::hash(&postcard::to_allocvec(&0).unwrap()));
        println!("1 {:?}", blake3::hash(&postcard::to_allocvec(&1).unwrap()));
        println!("2 {:?}", blake3::hash(&postcard::to_allocvec(&2).unwrap()));
        println!("3 {:?}", blake3::hash(&postcard::to_allocvec(&3).unwrap()));
        println!("4 {:?}", blake3::hash(&postcard::to_allocvec(&4).unwrap()));
        println!("5 {:?}", blake3::hash(&postcard::to_allocvec(&5).unwrap()));
        println!("6 {:?}", blake3::hash(&postcard::to_allocvec(&6).unwrap()));
        println!("7 {:?}", blake3::hash(&postcard::to_allocvec(&7).unwrap()));
        println!("8 {:?}", blake3::hash(&postcard::to_allocvec(&8).unwrap()));
        println!("9 {:?}", blake3::hash(&postcard::to_allocvec(&9).unwrap()));
        println!(
            "10 {:?}",
            blake3::hash(&postcard::to_allocvec(&10).unwrap())
        );
        println!(
            "11 {:?}",
            blake3::hash(&postcard::to_allocvec(&11).unwrap())
        );
        println!(
            "12 {:?}",
            blake3::hash(&postcard::to_allocvec(&12).unwrap())
        );
        println!(
            "13 {:?}",
            blake3::hash(&postcard::to_allocvec(&13).unwrap())
        );
        println!(
            "14 {:?}",
            blake3::hash(&postcard::to_allocvec(&14).unwrap())
        );
        println!(
            "15 {:?}",
            blake3::hash(&postcard::to_allocvec(&15).unwrap())
        );
        println!(
            "16 {:?}",
            blake3::hash(&postcard::to_allocvec(&16).unwrap())
        );
        println!(
            "17 {:?}",
            blake3::hash(&postcard::to_allocvec(&17).unwrap())
        );
        println!(
            "18 {:?}",
            blake3::hash(&postcard::to_allocvec(&18).unwrap())
        );
        println!(
            "19 {:?}",
            blake3::hash(&postcard::to_allocvec(&19).unwrap())
        );
        println!(
            "20 {:?}",
            blake3::hash(&postcard::to_allocvec(&20).unwrap())
        );
        println!(
            "21 {:?}",
            blake3::hash(&postcard::to_allocvec(&21).unwrap())
        );
        println!(
            "22 {:?}",
            blake3::hash(&postcard::to_allocvec(&22).unwrap())
        );
        println!(
            "23 {:?}",
            blake3::hash(&postcard::to_allocvec(&23).unwrap())
        );
        println!(
            "24 {:?}",
            blake3::hash(&postcard::to_allocvec(&24).unwrap())
        );
        println!(
            "25 {:?}",
            blake3::hash(&postcard::to_allocvec(&25).unwrap())
        );
        println!(
            "26 {:?}",
            blake3::hash(&postcard::to_allocvec(&26).unwrap())
        );
        println!(
            "27 {:?}",
            blake3::hash(&postcard::to_allocvec(&27).unwrap())
        );
        println!(
            "28 {:?}",
            blake3::hash(&postcard::to_allocvec(&28).unwrap())
        );
        println!(
            "29 {:?}",
            blake3::hash(&postcard::to_allocvec(&29).unwrap())
        );

        let t1 = blake3::hash(&postcard::to_allocvec(&14).unwrap());
        let t2 = blake3::hash(&postcard::to_allocvec(&26).unwrap());

        println!("14: {:?}", t1);
        println!("26: {:?}", t2);

        assert_eq!(Some((&26, &"26".to_owned(), false)), kad.search(&14));
    }
}
