use std::collections::{BTreeMap, BTreeSet};

use bdk_core::Merge;

#[test]
fn test_btree_map_merge() {
    let mut map1: BTreeMap<i32, &str> = BTreeMap::new();
    map1.insert(1, "a");
    let mut map2: BTreeMap<i32, &str> = BTreeMap::new();
    map2.insert(2, "b");

    map1.merge(map2);

    assert_eq!(map1.len(), 2);
    assert_eq!(map1.get(&1), Some(&"a"));
    assert_eq!(map1.get(&2), Some(&"b"));
}

#[test]
fn test_btree_set_merge() {
    let mut set1: BTreeSet<i32> = BTreeSet::new();
    set1.insert(1);
    let mut set2: BTreeSet<i32> = BTreeSet::new();
    set2.insert(2);

    set1.merge(set2);

    assert_eq!(set1.len(), 2);
    assert!(set1.contains(&1));
    assert!(set1.contains(&2));
}

#[test]
fn test_vec_merge() {
    let mut vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    vec1.merge(vec2);

    assert_eq!(vec1, vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_tuple_merge() {
    let mut tuple1 = (vec![1, 2], BTreeSet::from([3]));
    let tuple2 = (vec![3, 4], BTreeSet::from([4]));

    tuple1.merge(tuple2);

    assert_eq!(tuple1.0, vec![1, 2, 3, 4]);
    assert_eq!(tuple1.1.len(), 2);
    assert!(tuple1.1.contains(&3));
    assert!(tuple1.1.contains(&4));
}

#[test]
fn test_is_empty() {
    let map: BTreeMap<i32, i32> = BTreeMap::new();
    assert!(<BTreeMap<i32, i32>>::is_empty(&map));

    let set: BTreeSet<i32> = BTreeSet::new();
    assert!(<BTreeSet<i32>>::is_empty(&set));

    let vec: Vec<i32> = Vec::new();
    assert!(<Vec<i32>>::is_empty(&vec));
}

#[test]
fn test_take() {
    let mut map: BTreeMap<i32, i32> = BTreeMap::new();
    map.insert(1, 1);
    let taken_map = map.take();
    assert!(taken_map.is_some());
    assert!(map.is_empty());

    let mut set: BTreeSet<i32> = BTreeSet::new();
    set.insert(1);
    let taken_set = <BTreeSet<i32> as Merge>::take(&mut set);
    assert!(taken_set.is_some());
    assert!(set.is_empty());

    let mut vec: Vec<i32> = vec![1];
    let taken_vec = vec.take();
    assert!(taken_vec.is_some());
    assert!(vec.is_empty());
}

#[test]
fn test_btree_map_merge_conflict() {
    let mut map1: BTreeMap<i32, &str> = BTreeMap::new();
    map1.insert(1, "a");
    let mut map2: BTreeMap<i32, &str> = BTreeMap::new();
    map2.insert(1, "b");

    map1.merge(map2);

    assert_eq!(map1.len(), 1);
    assert_eq!(map1.get(&1), Some(&"b"));
}

#[test]
fn test_btree_set_merge_conflict() {
    let mut set1: BTreeSet<i32> = BTreeSet::new();
    set1.insert(1);
    let mut set2: BTreeSet<i32> = BTreeSet::new();
    set2.insert(1);

    set1.merge(set2);

    assert_eq!(set1.len(), 1);
    assert!(set1.contains(&1));
}

#[test]
fn test_vec_merge_duplicates() {
    let mut vec1 = vec![1, 2, 3];
    let vec2 = vec![3, 4, 5];

    vec1.merge(vec2);

    assert_eq!(vec1, vec![1, 2, 3, 3, 4, 5]);
}
