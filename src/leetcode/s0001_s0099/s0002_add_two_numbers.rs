use crate::utils::linked_list::ListNode;


#[allow(unused)]
pub struct Solution;


#[allow(unused)]
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut dummy = Some(Box::new(ListNode {val: 0, next: None}));
        let mut curr_node = &mut dummy;
        let mut n = 0;
        while l1.is_some() || l2.is_some() || n > 0{
            if let Some(mut v1) = l1 {
                n += v1.val;
                l1 = v1.as_mut().next.take();
            }
            if let Some(mut v2) = l2 {
                n += v2.val;
                l2 = v2.as_mut().next.take();
            }
            let val = n % 10;
            n = n / 10;
            curr_node.as_mut().unwrap().next = Some(Box::new(ListNode{val: val, next: None}));
            curr_node = &mut curr_node.as_mut().unwrap().next;
        }
        dummy.unwrap().next
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_two_numbers1(){
        let lh1 = ListNode::gen_linked_list(vec![2, 4, 3]);
        let lh2 = ListNode::gen_linked_list(vec![5, 6, 4]);
        let r = Solution::add_two_numbers(lh1, lh2);
        let rh = ListNode::gen_linked_list(vec![7, 0, 8]);
        ListNode::print_linked_list(r.clone());
        assert_eq!(r, rh);
    }
}