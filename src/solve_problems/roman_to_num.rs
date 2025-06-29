/*
Symbol       Value
I             1
V             5
X             10
L             50
C             100
D             500
M             1000

I 可以放在 V (5) 和 X (10) 的左边，来表示 4 和 9。
X 可以放在 L (50) 和 C (100) 的左边，来表示 40 和 90。
C 可以放在 D (500) 和 M (1000) 的左边，来表示 400 和 900。
*/

// pub fn roman_to_int(s: String) -> i32 {
//     let new_hashmap = std::collections::HashMap::from([
//         ('I', 1),
//         ('V', 5),
//         ('X', 10),
//         ('L', 50),
//         ('C', 100),
//         ('D', 500),
//         ('M', 1000),
//     ]);
//     let chars: Vec<char> = s.chars().collect();
//     let mut sum = 0;
//     let len = chars.len();
//     for i in 0..len {
//         let curr = new_hashmap[&chars[i]];
//         if i + 1 < len && curr < new_hashmap[&chars[i + 1]] {
//             sum -= curr;
//         } else {
//             sum += curr;
//         }
//     }
//     sum
// }

pub fn roman_to_int(s: String) -> i32 {
    fn value(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }
    let chars: Vec<char> = s.chars().collect();
    let mut sum = 0;
    for i in 0..chars.len() {
        let curr = value(chars[i]);
        if i + 1 < chars.len() && curr < value(chars[i + 1]) {
            sum -= curr;
        } else {
            sum += curr;
        }
    }
    sum
}
