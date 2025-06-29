use std::collections::HashSet;

pub fn remove_duplicates(nums: &mut Vec<i32>) -> (i32, Vec<i32>, HashSet<i32>) {
    let mut set: HashSet<i32> = std::collections::HashSet::new();
    let mut dumplicates: Vec<i32> = Vec::new();
    for num in nums {
        if set.contains(num) {
            dumplicates.push(*num);
        } else {
            set.insert(*num);
        }
    }
    (set.len() as i32, dumplicates, set)
}
