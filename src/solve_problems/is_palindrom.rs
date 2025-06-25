// 一种语句或词语，顺读和倒读都能成文，具有循环往复的特点
// Input: x = 121 Output: true
// x = -121     Output: false
// Could you solve it without converting the integer to a string?

// pub fn is_palindrome(x: i32) -> bool {
//     let s = x.to_string();
//     s.chars().eq(s.chars().rev())
// }
// pub fn is_palindrome(x: i32) -> bool {
//     // 负数和以0结尾的非0数不是回文
//     if x < 0 || (x % 10 == 0 && x != 0) {
//         return false;
//     }
//     /* else {
//         let mut vec = vec![];
//         for i in x.to_string().chars() {
//             vec.push(i);
//         }
//         let len = vec.len();
//         let mut left = 0;
//         let mut right = len - 1;
//         while left < right {
//             if vec[left] != vec[right] {
//                 return false;
//             }
//             left += 1;
//             right -= 1;
//         }
//         true
//     }
//     */
//     let mut x = x;
//     let mut rev = 0;
//     while x > rev {
//         rev = rev * 10 + x % 10;
//         x /= 10;
//     }
//     x == rev || x == rev / 10
// }

pub fn is_palindrome(x: i32) -> bool {
    let s = x.to_string();
    let bytes = s.as_bytes();
    let (mut left, mut right) = (0, bytes.len() - 1);
    while left < right {
        if bytes[left] != bytes[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}
