/*
示例 1：
输入：nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
输出：[1,2,2,3,5,6]
解释：需要合并 [1,2,3] 和 [2,5,6] 。
合并结果是 [1,2,2,3,5,6] ，其中斜体加粗标注的为 nums1 中的元素。

*/
pub fn merge_sort_array(
    nums1: &mut Vec<i32>,
    m: usize,
    nums2: &mut Vec<i32>,
    n: usize,
) -> Vec<i32> {
    let mut result: Vec<_> = nums1.iter().take(m).cloned().collect();
    result.extend(nums2.iter().take(n).cloned());
    result.sort();
    result
}
