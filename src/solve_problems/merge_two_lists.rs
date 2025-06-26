/*
Input: list1 = [1,2,4], list2 = [1,3,4]
Output: [1,1,2,3,4,4]
Input: list1 = [], list2 = []
Output: []
Input: list1 = [], list2 = [0]
Output: [0]
*/
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }

// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }
// pub fn merge_two_lists(
//     mut list1: Option<Box<ListNode>>,
//     mut list2: Option<Box<ListNode>>,
// ) -> Option<Box<ListNode>> {
//     let mut dummy = Box::new(ListNode::new(0));
//     let mut tail = &mut dummy;

//     while list1.is_some() && list2.is_some() {
//         let l1_val = list1.as_ref().unwrap().val;
//         let l2_val = list2.as_ref().unwrap().val;
//         if l1_val < l2_val {
//             let mut l1 = list1.take().unwrap();
//             list1 = l1.next.take();
//             tail.next = Some(l1);
//         } else {
//             let mut l2 = list2.take().unwrap();
//             list2 = l2.next.take();
//             tail.next = Some(l2);
//         }
//         tail = tail.next.as_mut().unwrap();
//     }
//     tail.next = if list1.is_some() { list1 } else { list2 };
//     dummy.next
// }

pub fn merge_two_lists(mut list1: Vec<i32>, mut list2: Vec<i32>) -> Vec<i32> {
    list1.append(&mut list2);
    list1.sort();
    list1
}
