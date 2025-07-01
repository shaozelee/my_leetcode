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

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn is_mirror(
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(left), Some(right)) => {
                let left = left.borrow();
                let right = right.borrow();
                left.val == right.val
                    && is_mirror(&left.left, &right.right)
                    && is_mirror(&left.right, &right.left)
            }
            _ => false,
        }
    }
    if let Some(root) = root {
        let root = root.borrow();
        is_mirror(&root.left, &root.right)
    } else {
        true
    }
}

#[test]
fn is_symmetric_tree() {
    use std::cell::RefCell;
    use std::rc::Rc;

    // 构造如下对称二叉树:
    //      1
    //     / \
    //    2   2
    //   / \ / \
    //  3  4 4  3
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let left = Rc::new(RefCell::new(TreeNode::new(2)));
    let right = Rc::new(RefCell::new(TreeNode::new(2)));
    let left_left = Rc::new(RefCell::new(TreeNode::new(3)));
    let left_right = Rc::new(RefCell::new(TreeNode::new(4)));
    let right_left = Rc::new(RefCell::new(TreeNode::new(4)));
    let right_right = Rc::new(RefCell::new(TreeNode::new(3)));

    left.borrow_mut().left = Some(left_left);
    left.borrow_mut().right = Some(left_right);
    right.borrow_mut().left = Some(right_left);
    right.borrow_mut().right = Some(right_right);

    root.borrow_mut().left = Some(left);
    root.borrow_mut().right = Some(right);

    assert!(is_symmetric(Some(root)));
}
