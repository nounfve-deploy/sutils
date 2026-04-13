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
{
    UPSERT!();
}

impl<K, V> MapExt<K, V> for BTreeMap<K, V>
where
    K: Eq + Ord,
{
    UPSERT!();
}

DEFINE!( UPSERT =
    fn upsert(&mut self, k: K, v: Option<V>) -> bool {
        if let Some(val) = v {
            self.insert(k, val);
            return true;
        }
        
        match self.remove(&k) {
            Some(_) => true,
            None => false,
        }
    }
);
