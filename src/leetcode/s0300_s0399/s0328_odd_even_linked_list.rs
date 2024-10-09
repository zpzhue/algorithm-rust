use crate::utils::linked_list::ListNode;

#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut node = dummy.as_mut().unwrap().next.as_mut();
        let mut odd_head = Some(Box::new(ListNode { val: 0, next: None }));
        let mut odd_node = odd_head.as_mut();
        while node.is_some() && node.as_ref().unwrap().next.is_some() {
            let mut next = node.as_mut().unwrap().next.take();
            let next_next = next.as_mut().unwrap().next.take();

            odd_node.as_mut().unwrap().next = next;
            odd_node = odd_node.unwrap().next.as_mut();
            node.as_mut().unwrap().next = next_next;
            if node.as_ref().unwrap().next.is_some(){
                node = node.unwrap().next.as_mut();
            }
        }
        if odd_head.as_ref().unwrap().next.is_some() {
            node.unwrap().next = odd_head.unwrap().next
        }
        dummy.unwrap().next
    }

    pub fn odd_even_list2 (head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none(){
            return head;
        }
        let mut curr = head;
        let mut is_odd = true;
        
        let mut odd_node = Some(Box::new(ListNode{val: 0, next: None}));
        let mut odd_ptr = odd_node.as_mut();
        let mut even_node = Some(Box::new(ListNode{val: 0, next: None}));
        let mut even_ptr = even_node.as_mut();

        while let Some(mut curr_node) = curr {
            curr = curr_node.next.take();
            if is_odd {
                odd_ptr.as_mut().unwrap().next = Some(curr_node);
                odd_ptr = odd_ptr.unwrap().next.as_mut();
            }else {
                even_ptr.as_mut().unwrap().next = Some(curr_node);
                even_ptr = even_ptr.unwrap().next.as_mut();
            }
            is_odd = !is_odd;
        }

        if let Some(node) = even_node.as_mut().unwrap().next.take() {
            odd_ptr.as_mut().unwrap().next = Some(node);
        }
        odd_node.unwrap().next
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_odd_even_list1() {
        let l = ListNode::gen_linked_list(vec![2, 1, 3, 5, 6, 4, 7]);
        let rl = ListNode::gen_linked_list(vec![2, 3, 6, 7, 1, 5, 4]);
        let r = Solution::odd_even_list2(l);
        ListNode::print_linked_list(r.clone());
        assert_eq!(rl, r);
    }

    #[test]
    fn test_odd_even_list2() {
        let l = ListNode::gen_linked_list(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        // let rl = ListNode::gen_linked_list(vec![2, 3, 6, 7, 1, 5, 4]);
        let r = Solution::odd_even_list(l);
        ListNode::print_linked_list(r.clone());
        // assert_eq!(rl, r);
    }
}
