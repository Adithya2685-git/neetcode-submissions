// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn pre(root: Option<&RefCell<TreeNode>>, a: &mut Vec<i32>){

        if let Some(node) = root{
            a.push(node.borrow().val);
        
            Self::pre(node.borrow().left.as_deref(), a);
            Self::pre(node.borrow().right.as_deref(), a);
        }


    }
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        
        let mut a = Vec::new();

        Self::pre(root.as_deref(), &mut a);
        a
    }
}
