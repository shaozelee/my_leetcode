pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // let mut new_map = std::collections::HashMap::new();
    // for (i, val) in nums.iter().enumerate() {
    //     let midval = target - val;

    //     if let Some(&index) = new_map.get(&midval) {
    //         return vec![index as i32, i as i32];
    //     }
    //     new_map.insert(val, i as i32);
    // }
    // vec![]

    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}
pub fn test_nums_target(nums: Vec<i32>, target: i32) -> bool {
    fn dfs(nums: &[i32], target: i32, idx: usize) -> bool {
        if target == 0 {
            return true;
        }
        if idx == nums.len() {
            return false;
        }
        // 选当前数 或 不选当前数
        dfs(nums, target, idx + 1) || dfs(nums, target - nums[idx], idx + 1)
    }
    dfs(&nums, target, 0)
}

/*
输入：nums = [3,2,4], target = 6
输出：[1,2]
*/
#[test]
fn test_two_sum() {
    let nums = vec![3, 2, 4];
    let target = 6;
    // println!("{:?}", two_sum(nums, target));
    assert_eq!(two_sum(nums, target), vec![1, 2]);
}

#[test]
fn test_test_nums_target() {
    let nums = vec![3, 2, 4];
    let target = 9;
    assert!(test_nums_target(nums.clone(), target)); // 3+2+4=9
    assert!(test_nums_target(nums.clone(), 7)); // 3+4=7
    assert!(test_nums_target(nums.clone(), 2)); // 2
    assert!(!test_nums_target(nums.clone(), 8)); // 不存在
}
