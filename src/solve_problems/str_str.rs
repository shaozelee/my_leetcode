/*
Input: haystack = "sadbutsad", needle = "sad"
Output: 0
Explanation: "sad" occurs at index 0 and 6.
The first occurrence is at index 0, so we return 0.

Input: haystack = "leetcode", needle = "leeto"
Output: -1
Explanation: "leeto" did not occur in "leetcode", so we return -1.

*/

pub fn str_str(haystack: String, neddle: String) -> i32 {
    if neddle.is_empty() {
        return 0;
    }
    // let n = haystack.len();
    // let m = neddle.len();
    // if m > n {
    //     return -1;
    // }
    // // 通过切片来确认
    // for i in 0..=n - m {
    //     if &haystack[i..i + m] == neddle {
    //         return i as i32;
    //     }
    // }
    // -1

    haystack.find(&neddle).map(|i| i as i32).unwrap_or(-1)
}
