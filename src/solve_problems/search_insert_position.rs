// Given a sorted array of distinct integers and a target value,
//  return the index if the target is found.
// If not, return the index where it would be if it were inserted in order.
// Input: nums = [1,3,5,6], target = 5
// Output: 2
// Input: nums = [1,3,5,6], target = 2
// Output: 1

pub fn search_insert(nums: Vec<i32>, target: i32) -> usize {
    // let reslut = nums.iter().position(|&x| x == target);
    // match reslut {
    //     Some(x) => x as i32,
    //     None => nums.partition_point(|&x| x < target) as i32,
    // }

    if let Some(idx) = nums.iter().position(|&x| x == target) {
        idx
    } else {
        nums.partition_point(|&x| x < target)
    }

    // let search_num = nums.binary_search(&target);
    // match search_num {
    //     Ok(index) => index,
    //     Err(index) => index,
    // }
}
