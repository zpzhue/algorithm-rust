// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }

    #[allow(unused)]
    pub fn print_linked_list(node: Option<Box<ListNode>>) {
        let mut head = Some(Box::new(ListNode {val: 0, next: node}));
        let mut curr = head.as_mut().unwrap().next.as_mut().unwrap();
        while curr.next.is_some() {
            print!("ListNode<{}>  -->  ", curr.val);
            curr = curr.next.as_mut().unwrap();
        }
        print!("ListNode<{}> \r\n", curr.val);
    }

    #[allow(unused)]
    pub fn gen_linked_list(list: Vec<i32>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut curr = dummy.as_mut().unwrap();
        curr.val = list[0];
        for i in 1..list.len() {
            let list_node = ListNode::new(list[i].clone());
            curr.next = Some(Box::new(list_node));
            curr = curr.next.as_mut().unwrap();
        }
        dummy
    }
}



#[cfg(test)]
mod test {
    use crate::utils::linked_list::ListNode;

    #[test]
    fn test_linked_list() {
        let l1 = ListNode::gen_linked_list(vec![2 ,5, 76, 32]);
        ListNode::print_linked_list(l1);
    }
}