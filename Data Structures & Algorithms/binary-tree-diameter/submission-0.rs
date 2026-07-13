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
    pub fn dfs(root: Option<&RefCell<TreeNode>>, max:&mut i32)-> i32{
        if let Some(node) = root{
            let left = Self::dfs(node.borrow().left.as_deref(), max);
            let right = Self::dfs(node.borrow().right.as_deref(), max);
            if left + right> *max{
                *max= left+ right;
            }
            left.max(right) +1
        }else{
            0
        }
    }
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max =0 ;
        Self::dfs(root.as_deref(), &mut max);
        max
    }
}
