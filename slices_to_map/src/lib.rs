use std::{cmp, collections::HashMap, hash::Hash};

pub fn slices_to_map<'a, T, U>(k: &'a [T], v: &'a [U]) -> HashMap<&'a T, &'a U>
where
    T: Hash + Eq,
{
    let len = cmp::min(k.len(), v.len());

    let mut map: HashMap<&T, &U> = HashMap::new();

    for i in 0..len {
        map.insert(&k[i], &v[i]);
    }

    map
}
