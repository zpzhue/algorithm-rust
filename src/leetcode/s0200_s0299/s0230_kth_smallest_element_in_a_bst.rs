use crate::utils::tree::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;

#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut res = 0;
        let mut level = 0;
        Solution::traverse(root, k, &mut level, &mut res);
        res
    }

    fn traverse(head: Option<Rc<RefCell<TreeNode>>>, k: i32, l: &mut i32, res: &mut i32) {
        if let Some(node) = head {
            let node_inner = node.borrow();
            Solution::traverse(node_inner.left.clone(), k, l, res);
            *l += 1;
            if k == *l {
                *res = node_inner.val
            }
            Solution::traverse(node_inner.right.clone(), k, l, res)
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_complie() {
        let r = Solution::kth_smallest(None, 1);
        println!("the function return result: {}", r);
    }
}