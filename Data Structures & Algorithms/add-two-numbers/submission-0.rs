// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>,mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        

        let mut dummy = Box::new(ListNode::new(0));

        let mut tail = &mut dummy;


        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry!= 0{

            let mut sum = carry;

            if let Some(n) = l1{
                sum+= n.val;
                l1 = n.next;
            }

            if let Some(n) = l2{
                sum+= n.val;
                l2 = n.next;
            }

            carry = sum/10;
            
            tail.next = Some(Box::new(ListNode::new(sum%10)));
            tail = tail.next.as_mut().unwrap();

        }

    dummy.next
    }
}
