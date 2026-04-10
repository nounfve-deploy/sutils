use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

use crate::DEFINE;

pub trait MapExt<K, V> {
    fn upsert(&mut self, k: K, v: Option<V>) -> bool;
}

impl<K, V> MapExt<K, V> for HashMap<K, V>
where
    K: Eq + Hash,
    V: PartialEq,
{
    UPSERT!();
}

impl<K, V> MapExt<K, V> for BTreeMap<K, V>
where
    K: Eq + Ord,
    V: PartialEq,
{
    UPSERT!();
}

DEFINE!( UPSERT =
    fn upsert(&mut self, k: K, v: Option<V>) -> bool {
        if self.get(&k) == v.as_ref() {
            return false;
        }
        match v {
            Some(val) => self.insert(k, val),
            None => self.remove(&k),
        };
        true
    }
);
