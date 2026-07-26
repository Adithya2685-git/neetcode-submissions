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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn dfs(root: Option<&RefCell<TreeNode>> ,k:i32, count: &mut i32, result: &mut i32){
        if let Some(node)= root{
            let left = Self::dfs(node.borrow().left.as_deref(), k, count, result);
            *count+=1;
            if *count == k{
                *result = node.borrow().val;
                return;
            }
            let right = Self::dfs(node.borrow().right.as_deref(),k, count, result);
            
        }
    }
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut count= 0;
        let mut result = -1;
        Self::dfs(root.as_deref(),k,&mut count, &mut result);
        result
    }
}
