#![allow(dead_code)]

// use solve_problems::inorder_traversal::*;
// use std::cell::RefCell;
// use std::rc::Rc;
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
    // let digits = vec![1, 9, 9];
    // let result = solve_problems::plus_one::plus_one(digits);
    // println!("{:?}", result)
    // let result = solve_problems::add_binary::add_binary("100".to_string(), "111".to_string());

    // println!("{}", result);
    // let x = 20;
    // println!("{}", solve_problems::my_sqar::my_sqrt(x));

    // let n = 45;
    // println!("{}", solve_problems::climb_stairs::climb_stairs(n));

    // let head = vec![1, 1, 2, 4, 5];
    // let mut result = solve_problems::delete_duplicates::delete_duplicates(head);
    // result.sort();
    // println!("{:?}", result);

    // let mut nums1 = vec![1, 2, 3, 45, 6, 7];
    // let mut nums2 = vec![8, 9, 10, 20];
    // let n = 3;
    // let m = 4;

    // let result = solve_problems::merge_sort_array::merge_sort_array(&mut nums1, m, &mut nums2, n);
    // println!("{:?}", result);

    // let root = Rc::new(RefCell::new(TreeNode::new(1)));
    // let left = Rc::new(RefCell::new(TreeNode::new(2)));
    // let right = Rc::new(RefCell::new(TreeNode::new(3)));
    // let left_left = Rc::new(RefCell::new(TreeNode::new(4)));
    // let left_right = Rc::new(RefCell::new(TreeNode::new(5)));

    // left.borrow_mut().left = Some(left_left);
    // left.borrow_mut().right = Some(left_right);
    // root.borrow_mut().left = Some(left);
    // root.borrow_mut().right = Some(right);

    // println!("递归: {:?}", inorder_traversal(&Some(root.clone()))); // [4, 2, 1, 3]
    // println!("迭代: {:?}", inorder_iterative(&Some(root))); // [4, 2, 1, 3]

    // let num1 = 200;
    // let num2 = 300;
    // println!("max_num: {}", num1.max(num2));

    let nums = vec![1, 1, 2, 2, 3, 3, 3];
    let mut max_count = 0;
    let mut majority = nums[0];
    for i in 0..nums.len() {
        let mut count = 0;

        for j in 0..nums.len() {
            // println!("nums[i] -> {} nums[j] -> {}", nums[i], nums[j]);
            if nums[i] == nums[j] {
                count += 1;
            }
        }
        if count > max_count {
            max_count = count;
            majority = nums[i];
        }
    }
    println!("{}", majority);
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
