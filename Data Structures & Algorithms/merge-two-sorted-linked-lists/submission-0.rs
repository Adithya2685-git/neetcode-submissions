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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr1 = list1;
        let mut curr2 = list2;
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while curr1.is_some() && curr2.is_some(){
            
            let v1= curr1.as_ref().unwrap().val;
            let v2= curr2.as_ref().unwrap().val;

            if v1< v2{
                let mut n1 = curr1.unwrap();
                curr1 = n1.next.take();
                tail.next= Some(n1); 
            }else{
                let mut n2 = curr2.unwrap();
                curr2 = n2.next.take();
                tail.next = Some(n2);

            }
            
            tail = tail.next.as_mut().unwrap();

        }

        if curr1.is_some(){
            tail.next= curr1;
        }else{
            tail.next = curr2;
        }

    dummy.next
    }
}
