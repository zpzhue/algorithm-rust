use crate::utils::linked_list::ListNode;

#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let n = lists.len();
        Self::dfs(&mut lists, 0, n)
    }

    fn dfs(list: &mut Vec<Option<Box<ListNode>>>, start: usize, end: usize) -> Option<Box<ListNode>> {
        if end - start <= 1 {
            if list.get(start).is_some() {
               return list[start].take()
            }
            return None
        }

        let mid = start + (end - start) / 2;
        println!("{}, {}, {}", start, mid, end);
        let mut left = Self::dfs(list, start, mid);
        let mut right = Self::dfs(list, mid, end);
        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = &mut dummy;
        
        while left.is_some() || right.is_some() {
            let mut next = None;
            if left.is_some() && (right.is_none() || left.as_ref().unwrap().val < right.as_ref().unwrap().val) {
                let t = left.as_mut().unwrap().next.take();
                next = left.take();
                left = t
            }else {
                let t = right.as_mut().unwrap().next.take();
                next = right.take();
                right = t
            }
            curr.next = next;
            curr = curr.next.as_mut().unwrap();
        }
        dummy.next
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_k_lists(){
        let arr = [[1,4,5],[1,3,4],[2, 6, 9]];
        let mut v = Vec::new();
        
        for a in arr {
            let l = ListNode::gen_linked_list(a.to_vec());
            v.push(l);
        }
        let res = Solution::merge_k_lists(v);
        ListNode::print_linked_list(res);
    }
}