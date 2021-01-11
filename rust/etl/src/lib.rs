use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut ret = BTreeMap::new();
    for (k, vev) in h {
        for  v in vev.iter() {
            let p = (*v).to_ascii_lowercase();
            ret.insert(p, *k);
        }
    }
    ret
}
