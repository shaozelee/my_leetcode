pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut new_map = std::collections::HashMap::new();

    for (i, val) in nums.iter().enumerate() {
        let midval = target - val;

        if let Some(&index) = new_map.get(&midval) {
            return vec![index as i32, i as i32];
        }
        new_map.insert(val, i as i32);
    }
    vec![]
}
