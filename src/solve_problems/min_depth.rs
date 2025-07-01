// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //  判断 root 的左右 node 如果有 depth + 1,递归判断
    match root {
        None => 0,
        Some(node) => {
            let left_depth = min_depth(node.borrow().left.clone());
            let right_depth = min_depth(node.borrow().right.clone());
            // if left_depth > right_depth {
            //     1 + left_depth
            // } else {
            //     1 + right_depth
            // }
            1 + left_depth.min(right_depth)
        }
    }
}

#[test]
fn test_min_depth() {
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let left = Rc::new(RefCell::new(TreeNode::new(2)));
    let left_left = Rc::new(RefCell::new(TreeNode::new(3)));
    let left_left_left = Rc::new(RefCell::new(TreeNode::new(4)));

    left_left.borrow_mut().left = Some(left_left_left);
    left.borrow_mut().left = Some(left_left);
    root.borrow_mut().left = Some(left);

    assert_eq!(min_depth(Some(root)), 1);
}
