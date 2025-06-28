// Input: digits = [1,9,9]
// Output: [2,0,0]
// Explanation: The array represents the integer 9.
// Incrementing by one gives 9 + 1 = 10.
// Thus, the result should be [1,0].

pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    for i in (0..digits.len()).rev() {
        if digits[i] < 9 {
            digits[i] += 1;
            return digits;
        }
        digits[i] = 0;
    }
    let mut result = vec![1];
    result.extend(digits);
    result
}
