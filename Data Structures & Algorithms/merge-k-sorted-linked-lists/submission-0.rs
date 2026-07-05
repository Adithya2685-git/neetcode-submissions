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

    pub fn merge(mut left: Option<Box<ListNode>>, mut right: Option<Box<ListNode>>) -> Option<Box<ListNode>>{
        
        let mut dummy = Box::new(ListNode::new(0));
        
        let mut curr = &mut dummy.next;

while left.is_some() && right.is_some() { 
            
            // ERROR B FIX: Peek at the values without taking ownership
            if left.as_ref().unwrap().val < right.as_ref().unwrap().val {
                
                // Now it is safe to unwrap ONLY the left node
                let mut l = left.unwrap();
                
                // ERROR C FIX: Use .take() to safely pull the rest of the list out
                left = l.next.take();
                
                // ERROR C FIX: Put the node INTO the slot
                *curr = Some(l);
                
            } else {
                
                // Safely unwrap ONLY the right node
                let mut r = right.unwrap();
                right = r.next.take();
                
                *curr = Some(r);
            }
            
            // ERROR C FIX: Advance 'curr' to point to the newly added node's empty 'next' slot
            curr = &mut curr.as_mut().unwrap().next;
        }

        // ERROR C FIX FOR CLEANUP:
        // You don't need `if let` here. `left` and `right` are already Options!
        // Just directly assign the remaining Option into the memory slot.
        if left.is_some() {
            *curr = left;
        } else {
            *curr = right;
        }

        dummy.next
    }


    pub fn divide(lists: &mut Vec<Option<Box<ListNode>>>, left: usize, right: usize)-> Option<Box<ListNode>>{
        if left == right{
            return lists[left].take();
        }

        let mid = left + (right-left)/2;

        let llist = Self::divide(lists, left,mid);
        let rlist = Self::divide(lists, mid+1,right);

        Self::merge(llist, rlist)
    }


    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {


        if lists.is_empty(){
            return None;
        }
        // recursive merge function.
        //should merge

        let len = lists.len();

        Self::divide(&mut lists, 0, len -1 )
    }
}
