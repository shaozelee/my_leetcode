#![allow(dead_code)]

mod solve_problems;

fn main() {
    // let result = solve_problems::two_sum::two_sum(vec![1, 2, 4, 5], 5);
    // println!("{:?}", result);

    // let num = 989;
    // let result = solve_problems::is_palindrom::is_palindrome(num);
    // println!("{}", result);

    // let result = solve_problems::roman_to_num::roman_to_int("MIVVVVI".to_string());
    // println!("{}", result);

    // let vec: Vec<String> = vec![
    //     "lsssa".to_string(),
    //     "lsssb".to_string(),
    //     "lsssc".to_string(),
    // ];

    // let str = solve_problems::longest_common_prefix::longest_common_prefix(vec);
    // println!("{}", str);

    // let str = "[]([)]".to_string();
    // let result = solve_problems::valid_parentheses::is_valid(str);
    // println!("{}", result);

    // let list1 = vec_to_list(vec![1, 2, 4]);
    // let list2 = vec_to_list(vec![1, 3, 4]);
    // let result = solve_problems::merge_two_lists::merge_two_lists(list1, list2);
    // let mut node = result;
    // while let Some(n) = node {
    //     print!("{} ", n.val);
    //     node = n.next;
    // }

    // let list1 = vec![1, 2, 3];
    // let list2 = vec![1, 5, 6];
    // let result = solve_problems::merge_two_lists::merge_two_lists(list1, list2);
    // println!("{:?}", result)

    // let mut vec = vec![1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5, 6, 7];
    // let result = solve_problems::remove_dumplicates::remove_duplicates(&mut vec);
    // println!("{:?}", result);

    // let mut vec = vec![1, 3, 4, 5, 6];
    // let val = 3;
    // let result = solve_problems::remove_element::remove_element(&mut vec, val);

    // println!("{:?}", result);

    // let haystack = "ssadbutsad".to_string();
    // let neddle = "sad".to_string();
    // let result = solve_problems::str_str::str_str(haystack, neddle);
    // println!("{}", result);
    // let nums = vec![1, 2, 3, 4, 6];
    // let target = 7;
    // let result = solve_problems::search_insert_position::search_insert(nums, target);
    // println!("{}", result);

    // let s = "i am study for rust 88888".to_string();

    // let result = solve_problems::lengeth_of_lat_word::length_of_last_word(s);
    // println!("{}", result);
    let digits = vec![1, 9, 9];
    let result = solve_problems::plus_one::plus_one(digits);
    println!("{:?}", result)
}

// fn vec_to_list(nums: Vec<i32>) -> Option<Box<solve_problems::merge_two_lists::ListNode>> {
//     let mut current = None;
//     for &num in nums.iter().rev() {
//         let mut node = Box::new(solve_problems::merge_two_lists::ListNode::new(num));
//         node.next = current;
//         current = Some(node);
//     }
//     current
// }
