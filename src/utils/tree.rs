// Definition for a binary tree node.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(unused)]
impl TreeNode {
    #[allow(unused)]
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    #[allow(unused)]
    pub fn to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::VecDeque;
        let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
        let mut queue = VecDeque::new();
        queue.push_back(head.as_ref().unwrap().clone());

        for children in vec[1..].chunks(2) {
            let parent = queue.pop_front().unwrap();
            if let Some(v) = children[0] {
                parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
            }
            if children.len() > 1 {
                if let Some(v) = children[1] {
                    parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                    queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
                }
            }
        }
        head
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_to_tree(){
        let node = TreeNode::to_tree(vec![Some(3), Some(9), None, None, Some(20), Some(15), Some(7)]);
        println!("result: {:?}", node)
    }
}
