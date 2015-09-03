use std::collections::hash_map::HashMap;
use std::hash::Hash;

struct Trie<K, V> where K: Eq+Hash+Clone, V: Clone {
    value: Option<V>,
    children: HashMap<K, Trie<K, V>>,
}

impl<K, V> Trie<K,V> where K: Eq+Hash+Clone, V: Clone {
    fn new() -> Trie<K, V> {
        Trie {
            value: None,
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, path: Vec<K>, v: V) {
        if path.is_empty() {
            match self.value {
                Some(_) => {
                    panic!("key exists")
                },
                None => {
                    self.value = Some(v);
                },
            }
            return;
        }

        self.children.entry(path[0].clone())
            .or_insert(Trie::new())
            .insert(path[1..].to_vec(), v)
    }

    fn fetch(&self, path: Vec<K>) -> Option<V> {
        match path.len() {
            0 => self.value.clone(),
            _ => self.children.get(&path[0])
                    .unwrap()
                    .fetch(path[1..].to_vec())
        }
    }
}


#[test]
fn fetch_works() {
    let mut t = Trie::new();
    t.insert(vec![1], 3);
    let f = t.fetch(vec![1]);
    assert_eq!(f, Some(3));
}

#[test]
fn deep_fetch_works() {
    let mut t = Trie::new();
    t.insert(vec![1,2,3], 4);

    let v1 = t.fetch(vec![1]);
    assert_eq!(v1, None);

    let v2 = t.fetch(vec![1,2,3]);
    assert_eq!(v2, Some(4));
}

#[test]
#[should_panic]
fn insert_panics_if_exists() {
    let mut t = Trie::new();
    t.insert(vec![1], 3);
    t.insert(vec![1], 4);
}

#[test]
fn insert_works_if_none() {
    let mut t = Trie::new();
    t.insert(vec![1,2,3], 4);
    t.insert(vec![1,2], 5);

    assert_eq!(t.fetch(vec![1]), None);
    assert_eq!(t.fetch(vec![1,2]), Some(5));
    assert_eq!(t.fetch(vec![1,2,3]), Some(4));
}

#[test]
fn works_with_multiple_types() {
    let mut t = Trie::new();
    t.insert(vec![1,2,3], "hello");
    t.insert(vec![1,3,4], "goodbye");

    assert_eq!(t.fetch(vec![1]), None);
    assert_eq!(t.fetch(vec![1,2,3]), Some("hello"));
    assert_eq!(t.fetch(vec![1,3,4]), Some("goodbye"));
}
