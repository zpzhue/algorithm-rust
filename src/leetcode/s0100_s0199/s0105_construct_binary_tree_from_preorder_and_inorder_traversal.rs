use crate::utils::tree::TreeNode;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {

    fn build(preorder: &Vec<i32>, pre_start: usize, pre_end: usize, 
            inorder: &Vec<i32>, in_start: usize, in_end: usize,
            map: &HashMap<&i32, usize>
        ) -> Option<Rc<RefCell<TreeNode>>> {
        if pre_start > pre_end {
            return None;
        }else if preorder.len() == 1 {
            return Some(Rc::new(RefCell::new(TreeNode::new(preorder[pre_start]))));
        }

        // root 节点的值
        let root_val = preorder[pre_start];
        let in_root_index = *map.get(&root_val).unwrap();
        let left_size = in_root_index - in_start;

        // 构建root节点
        let mut node = Some(Rc::new(RefCell::new(TreeNode::new(root_val))));
        node.as_mut().unwrap().borrow_mut().left = Solution::build(preorder, pre_start + 1, pre_start + left_size, 
                                    inorder, in_start, in_root_index - 1, map);
        node.as_mut().unwrap().borrow_mut().right = Solution::build(preorder, pre_start + left_size + 1, pre_end, 
                                    inorder, in_root_index + 1, in_end, map);
        node
    }

    fn build2(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }
        
        let root_val = preorder[0];
        // let root_index = inorder.iter().position(|&v| v == root_val).unwrap();
        let mut root_index = 0;
        for i in 0..inorder.len() {
            if inorder[i] == root_val {
                root_index = i;
                break;
            }
        }
        Some(Rc::new(RefCell::new(TreeNode {
            val: root_val,
            left: Solution::build2(&preorder[1..root_index + 1], &inorder[0..root_index]),
            right: Solution::build2(&preorder[root_index + 1 ..], &inorder[root_index + 1..])
        })))
    }

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let map = inorder.iter()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect::<HashMap<&i32, usize>>();
        Solution::build(&preorder, 0, preorder.len(), 
                        &inorder, 0, inorder.len(), &map)
        // Solution::build2(&preorder[..], &inorder[..])
    }



}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_build_tree(){
        let _ = Solution::build_tree(vec![1], vec![1]);
    }
}