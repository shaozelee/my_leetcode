// 给你两个二进制字符串 a 和 b ，以二进制字符串的形式返回它们的和。
// 示例 1：
// 输入:a = "11", b = "1"
// 输出："100"
// 示例 2：
// 输入：a = "1010", b = "1011"
// 输出："10101"    0 + 1 = 1  1+1 = 10
pub fn add_binary(a: String, b: String) -> String {
    let mut result = String::new();
    let mut carry = 0;
    let mut a_iter = a.chars().rev();
    let mut b_iter = b.chars().rev();

    loop {
        let digit_a = a_iter.next().and_then(|c| c.to_digit(2));
        let digit_b = b_iter.next().and_then(|c| c.to_digit(2));

        if digit_a.is_none() && digit_b.is_none() && carry == 0 {
            break;
        }

        let sum = digit_a.unwrap_or(0) + digit_b.unwrap_or(0) + carry;
        result.push(std::char::from_digit(sum % 2, 10).unwrap());
        carry = sum / 2;
    }

    result.chars().rev().collect()
}
