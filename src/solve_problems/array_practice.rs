use std::collections::{HashMap, HashSet};

pub fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

pub fn max(nums: &[i32]) -> Option<i32> {
    nums.iter().cloned().max()
}

pub fn min(nums: &[i32]) -> Option<i32> {
    nums.iter().cloned().min()
}

pub fn contains(nums: &[i32], target: i32) -> bool {
    nums.iter().any(|&x| x == target)
}
/*
输入：[7,1,5,3,6,4]
5
*/
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = i32::MAX;
    let mut max_profit = 0;
    for price in prices {
        // 找到历史最低价格，用当前价格减去历史最低从而得到最大利润
        if price < min_price {
            min_price = price;
        } else if price - min_price > max_profit {
            max_profit = price - min_price;
        }
    }
    max_profit
}

// 输入：nums = [1,1,2,2,3]
// 输出：3

pub fn single_number(nums: Vec<i32>) -> i32 {
    // nums.into_iter().fold(0, |acc, x| acc ^ x)
    for i in 0..nums.len() {
        let mut count = 0;
        for j in 0..nums.len() {
            if nums[i] == nums[j] {
                count += 1;
            }
        }
        if count == 1 {
            return nums[i];
        }
    }
    0
}
/*
输入：nums = [2,2,1,1,1,2,2]
输出：2
*/
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut max_count = 0;
    let mut result = nums[0];
    for num in nums {
        let count = map.entry(num).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            result = num;
        }
    }
    result
}
/*
输入：nums = [1,2,3,1]
输出：true
解释：
元素 1 在下标 0 和 3 出现。
*/
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    for num in nums {
        // HashSet::insert 如果元素已存在会返回 false 不能进行插入，否则返回插入返回 true
        if !set.insert(num) {
            return true;
        }
    }
    false
    // let mut map = HashMap::new();
    // for num in nums {
    //     let count = map.entry(num).or_insert(0);
    //     *count += 1;
    //     if *count > 1 {
    //         return true;
    //     }
    // }
    // false
    // for i in 0..nums.len() {
    //     let mut count = 0;
    //     for j in 0..nums.len() {
    //         if nums[i] == nums[j] {
    //             count += 1;
    //             if count >= 2 {
    //                 return true;
    //             }
    //         }
    //     }
    // }
    // false
}

/*
给你一个整数数组 nums 和一个整数 k ，判断数组中是否存在两个不同的索引 i 和 j ,
满足 nums[i] == nums[j] 且 abs(i - j) <= k 。如果存在，返回 true ；否则，返回 false
*/
pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut dictionary = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        if let Some(&prev_index) = dictionary.get(&num) {
            if i as i32 - prev_index <= k {
                return true;
            }
        }
        dictionary.insert(num, i as i32);
    }
    false
}
/*
输入：nums = [0,1,2,4,5,7]
输出：["0->2","4->5","7"]
数组中只要是有连续的就是输出这一段范围，不是连续的就直接输出这个数字
*/
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut new_vec: Vec<String> = vec![];
    if nums.is_empty() {
        return new_vec;
    }
    let mut start = nums[0];
    let mut end = nums[0];
    for &num in nums.iter().skip(1) {
        if num == end + 1 {
            end = num;
        } else {
            if start == end {
                new_vec.push(format!("{}", start));
            } else {
                new_vec.push(format!("{}->{}", start, end));
            }
            start = num;
            end = num;
        }
    }
    if start == end {
        new_vec.push(format!("{}", start));
    } else {
        new_vec.push(format!("{}->{}", start, end));
    }
    new_vec
}

/*
输入：nums = [9,6,4,2,3,5,7,0,1]
输出：8
给定一个包含 [0, n] 中 n 个数的数组 nums ，找出 [0, n] 这个范围内没有出现在数组中的那个数。
*/
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    for i in 0..=n {
        if !nums.contains(&i) {
            return i as i32;
        }
    }
    0
    // let n = nums.len();
    // let mut res = n as i32;
    // for (i, &num) in nums.iter().enumerate() {
    //     res ^= (i as i32) ^ num;
    // }
    // res
}

pub fn all_missing_number(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len() as i32;
    let mut new_vec = vec![];
    for i in 0..=n {
        if !nums.contains(&i) {
            new_vec.push(i);
        }
    }
    new_vec
}

/*
示例 1:
输入: nums = [0,1,0,3,12]
输出: [1,3,12,0,0]

示例 2:
输入: nums = [0]
输出: [0]
*/
pub fn move_zeroes(nums: &mut Vec<i32>) {
    nums.sort_by(|a, b| match (a == &0, b == &0) {
        (true, true) => std::cmp::Ordering::Equal, // 两元素均为0：顺序不变
        (true, false) => std::cmp::Ordering::Greater, // a为0、b非零：a排到b后
        (false, true) => std::cmp::Ordering::Less, // a非零、b为0：a排到b前
        _ => a.cmp(b),                             // 两元素均非零：正常升序比较
    });
}
/*
输入：nums1 = [1,2,2,1], nums2 = [2,2]
输出：[2]
示例 2：
输入：nums1 = [4,9,5], nums2 = [9,4,9,8,4]
输出：[9,4]
解释：[4,9] 也是可通过的
*/
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let set1: HashSet<i32> = nums1.iter().cloned().collect();
    let set2: HashSet<i32> = nums2.iter().cloned().collect();
    set1.intersection(&set2).copied().collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(&[1, 2, 3]), 6);
    }

    #[test]
    fn test_max() {
        assert_eq!(max(&[1, 2, 3]), Some(3));
    }

    #[test]
    fn test_min() {
        assert_eq!(min(&[1, 2, 3]), Some(1));
    }

    #[test]
    fn test_contains() {
        assert!(contains(&[1, 2, 3], 2));
        assert!(!contains(&[1, 2, 3], 4));
    }
    #[test]
    fn test_max_profit() {
        let vec = vec![7, 1, 5, 3, 6, 8];
        assert_eq!(max_profit(vec), 7);
    }
    #[test]
    fn test_single_number() {
        let nums = vec![1, 1, 2, 2, 3, 3, 4];
        assert_eq!(single_number(nums), 4);
    }
    #[test]
    fn test_majority_element() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2, 3, 3, 3, 3, 3];
        assert_eq!(majority_element(nums), 3);
    }
    #[test]
    fn test_contains_duplicate() {
        let nums = vec![1, 2, 3, 4, 5, 6, 1];
        assert!(contains_duplicate(nums));
    }
    #[test]
    fn test_contains_nearby_duplicate() {
        let nums = vec![1, 2, 3, 1];
        let k = 3;
        assert!(contains_nearby_duplicate(nums, k));
    }
    #[test]
    fn test_summary_ranges() {
        let nums = vec![1, 2, 3, 4, 6, 7, 8];
        assert_eq!(summary_ranges(nums), vec!["1->4", "6->8"]);
    }
    #[test]
    fn test_missing_number() {
        let nums = vec![0, 1, 2, 3, 4, 5, 7, 8];
        assert_eq!(missing_number(nums), 6);
    }
    #[test]
    fn test_move_zeroes() {
        let mut nums = vec![0, 1, 2, 3, 4, 5, 7, 8];
        println!("{:?}", move_zeroes(&mut nums));
    }
}
