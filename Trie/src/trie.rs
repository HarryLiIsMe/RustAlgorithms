use std::marker::PhantomData;

pub struct Trie<V> {
    _marker: PhantomData<V>,
}

impl<V> Trie<V> {
    fn get(&self, key: &[u8]) -> Option<&V> {
        Option::None
    }

    fn insert(&self, key: &[u8], val: V) -> Option<V> {
        Option::None
    }

    fn remove(&self, key: &[u8]) -> Option<V> {
        Option::None
    }

    fn len(&self) -> usize {
        0
    }
}