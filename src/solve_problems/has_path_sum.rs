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
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    match root {
        None => false,
        Some(node) => {
            let node = node.borrow();
            let val = node.val;
            let left = node.left.clone();
            let right = node.right.clone();

            // 如果是叶子节点，判断路径和是否等于 target_sum
            if left.is_none() && right.is_none() {
                return val == target_sum;
            }

            // 递归判断左右子树
            has_path_sum(left, target_sum - val) || has_path_sum(right, target_sum - val)
        }
    }
}

#[test]
fn test_has_path_sum() {
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let left = Rc::new(RefCell::new(TreeNode::new(2)));
    let left_left = Rc::new(RefCell::new(TreeNode::new(3)));
    let left_left_left = Rc::new(RefCell::new(TreeNode::new(4)));
    let left_left_right = Rc::new(RefCell::new(TreeNode::new(5)));

    left_left.borrow_mut().left = Some(left_left_left);
    left_left.borrow_mut().right = Some(left_left_right);
    left.borrow_mut().left = Some(left_left);
    root.borrow_mut().left = Some(left);

    assert!(has_path_sum(Some(root.clone()), 10));
    assert!(has_path_sum(Some(root.clone()), 11));
    // assert!(has_path_sum(Some(root.clone()), 12));
}
