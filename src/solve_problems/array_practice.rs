use std::collections::HashMap;

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
}
