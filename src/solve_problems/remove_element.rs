/*
Input: nums = [0,1,2,2,3,0,4,2], val = 2
Output: 5, nums = [0,1,4,0,3,_,_,_]

Input: nums = [3,2,2,3], val = 3
Output: 2, nums = [2,2,_,_]

*/

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> (i32, &mut Vec<i32>) {
    // let new_num: Vec<i32> = nums.iter().cloned().filter(|&x| x != val).collect();
    nums.retain(|&x| x != val);
    (nums.len() as i32, nums)
}
