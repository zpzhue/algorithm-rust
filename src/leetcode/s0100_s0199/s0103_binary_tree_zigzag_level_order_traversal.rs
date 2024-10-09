use crate::utils::tree::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }

        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut is_left = true;

        while !queue.is_empty() {
            let n = queue.len();
            let mut arr = Vec::with_capacity(n);
            for _ in 0..n {                
                if let Some(node) = queue.pop_front().unwrap() {
                    let mut node_inner= node.borrow_mut();
                    if is_left {
                        arr.push(node_inner.val);
                    }else {
                        arr.insert(0, node_inner.val);
                    }

                    if node_inner.left.is_some() {
                        queue.push_back(node_inner.left.take());
                    }
                    if node_inner.right.is_some() {
                        queue.push_back(node_inner.right.take());
                    }
                }
            }
            is_left = !is_left;
            result.push(arr);
        }
        result
    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zigzag_level_order() {
        let r = Solution::zigzag_level_order(None);
        println!("r={:?}", r);
    }
}