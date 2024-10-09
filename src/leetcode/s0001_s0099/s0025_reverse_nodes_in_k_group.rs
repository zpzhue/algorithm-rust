use crate::utils::linked_list::ListNode;
#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode {val: 0, next: head}));
        let mut head = dummy_head.as_mut();
        'outer:loop {
            let mut start = head.as_mut().unwrap().next.take();
            if start.is_none() {
                break 'outer;
            }
            let mut end = start.as_mut();
            for _ in 0..(k - 1) {
                end = end.unwrap().next.as_mut();
                if end.is_none(){
                    head.as_mut().unwrap().next = start;
                    break 'outer;
                }
            }
            let tail = end.as_mut().unwrap().next.take();
            let end = Solution::reverse_group(start, tail);
            head.as_mut().unwrap().next = end;
            for _ in 0..k {
                head = head.unwrap().next.as_mut();
            }
        }
        dummy_head.unwrap().next
    }

    fn reverse_group(mut head: Option<Box<ListNode>>, tail: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre  = tail;
        let mut curr = head;

        while let Some(mut curr_inner) = curr {
            let next = curr_inner.next.take();
            curr_inner.next = pre.take();
            pre = Some(curr_inner);
            curr = next;
        }
        pre
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_k_group(){
        let l = ListNode::gen_linked_list(vec![1,2,3,4,5]);
        let r = Solution::reverse_k_group(l, 2);
        ListNode::print_linked_list(r);
    }
}