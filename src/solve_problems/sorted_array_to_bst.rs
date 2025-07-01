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
pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let mid = nums.len() / 2;
        let mut node = TreeNode::new(nums[mid]);
        node.left = helper(&nums[..mid]);
        node.right = helper(&nums[mid + 1..]);
        Some(Rc::new(RefCell::new(node)))
    }
    helper(&nums)
}

#[test]
fn test_sorted_array_to_bst() {
    let nums = vec![-10, -3, 0, 5, 9];
    let tree = sorted_array_to_bst(nums.clone());

    // 中序遍历
    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
        if let Some(n) = node {
            let n = n.borrow();
            inorder(&n.left, vals);
            vals.push(n.val);
            inorder(&n.right, vals);
        }
    }

    let mut result = Vec::new();
    inorder(&tree, &mut result);

    assert_eq!(result, nums);
}
