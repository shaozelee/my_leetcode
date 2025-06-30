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
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(node_p), Some(node_q)) => {
            let p = node_p.borrow();
            let q = node_q.borrow();
            p.val == q.val
                && is_same_tree(p.left.clone(), q.left.clone())
                && is_same_tree(p.right.clone(), q.right.clone())
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn tree_from_vec(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = vec![root.clone()];
        let mut i = 1;
        while i < vals.len() {
            let node = queue.remove(0);
            if let Some(Some(val)) = vals.get(i) {
                let left = Rc::new(RefCell::new(TreeNode::new(*val)));
                node.borrow_mut().left = Some(left.clone());
                queue.push(left);
            }
            i += 1;
            if let Some(Some(val)) = vals.get(i) {
                let right = Rc::new(RefCell::new(TreeNode::new(*val)));
                node.borrow_mut().right = Some(right.clone());
                queue.push(right);
            }
            i += 1;
        }
        Some(root)
    }

    #[test]
    fn test_is_same_tree() {
        let t1 = tree_from_vec(&[Some(1), Some(2), Some(3)]);
        let t2 = tree_from_vec(&[Some(1), Some(2), Some(3)]);
        assert!(is_same_tree(t1.clone(), t2.clone()));

        let t3 = tree_from_vec(&[Some(1), Some(2)]);
        let t4 = tree_from_vec(&[Some(1), None, Some(2)]);
        assert!(!is_same_tree(t3, t4));

        let t5 = tree_from_vec(&[Some(1), Some(2), Some(1)]);
        let t6 = tree_from_vec(&[Some(1), Some(1), Some(2)]);
        assert!(!is_same_tree(t5, t6));
    }
}
