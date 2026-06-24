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
    pub fn post(root: Option<Rc<RefCell<TreeNode>>>, result:&mut Vec<i32>){

        if let Some(node) = root{
        

        Self::post(node.borrow().left.clone(), result);
        Self::post(node.borrow().right.clone(), result);
        
        result.push(node.borrow().val);

        }
    }   
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        
        let mut result: Vec<i32> = Vec::new();

        Self:: post(root,&mut result);
        result
    }   
}
