use std::cell::RefCell;
use std::rc::Rc;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn inorder_traversal(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = vec![];
    inorder(root, &mut result);
    result
}

fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    match node {
        Some(n) => {
            let n = n.borrow();
            // 1. 遍历左子树
            inorder(&n.left, result);
            // 2. 访问根节点
            result.push(n.val);
            // 3. 遍历右子树
            inorder(&n.right, result);
        }
        None => (),
    }
}
