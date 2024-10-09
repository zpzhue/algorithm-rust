/**
100. 相同的树
给你两棵二叉树的根节点 p 和 q ，编写一个函数来检验这两棵树是否相同。
如果两个树在结构上相同，并且节点具有相同的值，则认为它们是相同的。


示例 1：
输入：p = [1,2,3], q = [1,2,3]
输出：true


示例 2：
输入：p = [1,2], q = [1,null,2]
输出：false


示例 3：
输入：p = [1,2,1], q = [1,1,2]
输出：false
 

提示：
两棵树上的节点数目都在范围 [0, 100] 内
-104 <= Node.val <= 104

通过次数439,782提交次数734,287
*/


use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::tree::TreeNode;


#[allow(unused)]
pub struct Solution;


#[allow(unused)]
impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(&p, &q)
    }

    pub fn dfs(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }

        if p.is_none() || q.is_none() {
            return false;
        }

        let p1 = p.as_ref().unwrap().borrow_mut();
        let q1 = q.as_ref().unwrap().borrow_mut();

        p1.val == q1.val && Self::dfs(&p1.left, &q1.left) && Self::dfs(&p1.right, &q1.right)
    }
}

