pub fn optadd(a: Option<usize>, b: Option<usize>) -> Option<usize> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a + b),
        (None, Some(_b)) => None,
        (Some(a), None) => Some(a),
        _ => None,
    }
}

pub fn find<T, S>(array: S, key: T) -> Option<usize>
where
    T: PartialEq + PartialOrd,
    S: AsRef<[T]>,
{
    let array = array.as_ref();
    let len = array.len();
    let mid = len / 2;
    if len >= 1 {
        if array[mid] == key {
            Some(mid)
        } else if array[mid] > key {
            find(&array[0..mid], key)
        } else {
            optadd(find(&array[mid + 1..], key), Some(mid + 1))
        }
    } else {
        None
    }
}
