[![Latest Version](https://img.shields.io/badge/crates.io-v0.0.0-green.svg)](https://crates.io/crates/rckad)

# rckad
Efficient and flexible S/Kademlia implementation.

```rust
use rckad::KadTree;

fn main() {
    let mut kad = KadTree::new(1, "a"); // self_key, self_value
    
    kad.add(2, "b");
    kad.add(3, "c");
    kad.add(4, "e");
    
    assert_eq!(Some((&"b", true)), kad.search(&2));
    assert_eq!(true, kad.contains(&2));

    kad.remove(&2);
    assert_eq!(false, kad.contains(&2));
}

```

- [Kademlia](https://pdos.csail.mit.edu/~petar/papers/maymounkov-kademlia-lncs.pdf)
- [S/Kademlia](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.68.4986&rep=rep1&type=pdf)


## License

This project is licensed under， it's your choice.

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

