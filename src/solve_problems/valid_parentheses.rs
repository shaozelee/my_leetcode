/**
 * {} () []
 *
 * 左括号必须用相同类型的括号闭合。        
 * 左括号必须按正确的顺序闭合。
 * 每个右括号都有与之对应的相同类型的左括号 -- 用 % 2 来做
 *
 * 找个 hashmap 存储  举列子 '[',3 ']',3  '{',1  '}',1  
 * -- 找到左边出现 ([{ 几次 再 找 右边出现 }]) 分别出现几次,次数一样返回true
 * 确定顺序？ 下面的方法不行，确定顺序需要用到栈
 */

// pub fn is_valid(s: String) -> bool {
//     let len = s.len();
//     if len % 2 != 0 {
//         return false;
//     } else {
//         let mut new_hashmap = std::collections::HashMap::new();
//         // 遍历 s 对每个[({})]元素 计算次数
//         for c in s.chars() {
//             //  获取 new_hashmap 中的值如果有 +1 没有 初始化
//             *new_hashmap.entry(c).or_insert(0) += 1;
//         }
//         // 判断 是否一样
//         if new_hashmap.get(&'(') == new_hashmap.get(&')')
//             && new_hashmap.get(&'[') == new_hashmap.get(&']')
//             && new_hashmap.get(&'{') == new_hashmap.get(&'}')
//         {
//             return true;
//         } else {
//             return false;
//         }
//     }
// }

pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            // []([)] 顺序不对分析
            // [[()]] 顺序正确分析
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}
