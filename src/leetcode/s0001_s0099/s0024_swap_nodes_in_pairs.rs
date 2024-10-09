use crate::utils::linked_list::ListNode;
#[allow(unused)]
struct Solution;

#[allow(unused)]
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode{ val: 0, next: head }));
        let mut curr = dummy.as_mut().unwrap();

        while curr.next.is_some() && curr.next.as_ref().unwrap().next.is_some() {
            curr.next = {
                let mut b = curr.next.as_mut().unwrap().next.take();
                curr.next.as_mut().unwrap().next = b.as_mut().unwrap().next.take();
                let a = curr.next.take();
                b.as_mut().unwrap().next = a;
                b
            };
            curr = curr.next.as_mut().unwrap().next.as_mut().unwrap();
        }
        dummy.unwrap().next
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_swap_pairs() {
        let l = ListNode::gen_linked_list(vec![2, 6, 17, 5, 90]);
        let l =Solution::swap_pairs(l);
        ListNode::print_linked_list(l);
    }
}