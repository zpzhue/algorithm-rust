/**
111. 二叉树的最小深度
给定一个二叉树，找出其最小深度。

最小深度是从根节点到最近叶子节点的最短路径上的节点数量。

说明：叶子节点是指没有子节点的节点。

 
示例 1：
输入：root = [3,9,20,null,null,15,7]
输出：2

示例 2：
输入：root = [2,null,3,null,4,null,5,null,6]
输出：5
 

提示：
树中节点数的范围在 [0, 105] 内
-1000 <= Node.val <= 1000

通过次数531,967提交次数1,025,447
*/

use crate::utils::tree::TreeNode;


use std::rc::Rc;
use std::cell::RefCell;


#[allow(unused)]
pub struct Solution;


#[allow(unused)]
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root)
    }

    
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let node = root.as_ref().unwrap().borrow_mut();
        if node.left.is_none() {
            return 1 + Self::dfs(&node.right);
        }

        if node.right.is_none() {
            return 1 + Self::dfs(&node.left);
        }

        let l = Self::dfs(&node.left);
        let r = Self::dfs(&node.right);

        l.min(r) + 1
    }
}