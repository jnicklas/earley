use std::collections::BTreeMap;
use std::collections::btree_map::IntoIter as BTreeIter;

pub trait GroupByExt<I> {
    fn group_by<K, F>(self, f: F) -> BTreeIter<K, Vec<I>> where
        F: for<'a> Fn(&'a I) -> K,
        K: Ord;
}

impl<T, I> GroupByExt<I> for T where T: Iterator<Item=I> {
    fn group_by<K, F>(self, f: F) -> BTreeIter<K, Vec<I>> where
        F: for<'a> Fn(&'a I) -> K,
        K: Ord
    {
        let mut map = BTreeMap::new();
        for item in self {
            let hash = f(&item);
            map.entry(hash).or_insert(vec![]).push(item);
        }
        map.into_iter()
    }
}

#[test]
fn test_group_by() {
    let map: BTreeMap<u32, Vec<u32>> = (1..7).group_by(|i| i%3).collect();

    assert_eq!(map[&0], vec![3, 6]);
    assert_eq!(map[&1], vec![1, 4]);
    assert_eq!(map[&2], vec![2, 5]);
}
