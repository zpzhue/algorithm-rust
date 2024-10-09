use crate::utils::tree::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;


#[allow(unused)]
fn traversal(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    if let Some(node) = root {
        let node = node.borrow();
        // 前序便利
        traversal(node.left.clone(), result);
        // 中序便利
        result.push(node.val);
        traversal(node.right.clone(), result);
        // 后序遍历
    }
}

#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        traversal(root, &mut result);
        result
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test(){
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let r1 = Rc::new(RefCell::new(TreeNode::new(2)));
        let l2 = Rc::new(RefCell::new(TreeNode::new(3)));

        r1.borrow_mut().left = Some(l2);
        root.borrow_mut().right = Some(r1);
        assert_eq!(Solution::inorder_traversal(Some(root)), vec![1, 3, 2])
    }
}