use std::collections::HashSet;

// change  vec to hashset
pub fn delete_duplicates(head: Vec<i32>) -> Vec<i32> {
    let set: HashSet<_> = head.into_iter().collect();
    set.into_iter().collect()
}
