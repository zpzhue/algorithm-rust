/**
110. 平衡二叉树
给定一个二叉树，判断它是否是高度平衡的二叉树。

本题中，一棵高度平衡二叉树定义为：

一个二叉树每个节点 的左右两个子树的高度差的绝对值不超过 1 。

 
示例 1：
输入：root = [3,9,20,null,null,15,7]
输出：true


示例 2：
输入：root = [1,2,2,3,3,null,null,4,4]
输出：false


示例 3：
输入：root = []
输出：true
 

提示：
树中的节点数在范围 [0, 5000] 内
-104 <= Node.val <= 104

通过次数482,077提交次数837,897
*/

use crate::utils::tree::TreeNode;


use std::rc::Rc;
use std::cell::RefCell;


#[allow(unused)]
pub struct Solution;


#[allow(unused)]
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(&root) >= 0
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> i32{
        if root.is_none(){
            return 0;
        }

        let node = root.as_ref().unwrap().borrow_mut();

        let l = Self::dfs(&node.left);
        let r = Self::dfs(&node.right);

        if l == -1 || r == -1 || (l - r).abs() > 1 {
            return -1;
        }
        l.max(r) + 1
    }
}