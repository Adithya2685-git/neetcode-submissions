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

    pub fn dfs(root: Option<&RefCell<TreeNode>>, balanced: &mut bool) -> i32{

        if let Some (node) = root{

            let left = Self::dfs(node.borrow().left.as_deref(),balanced);
            let right = Self::dfs(node.borrow().right.as_deref(),balanced);

            if left - right >1 || left - right < -1 {
                *balanced = false;
            }
            return left.max(right) + 1;

        }
        0
    }
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut balanced = true;
        let depth = Self::dfs(root.as_deref(),&mut balanced);

        balanced
    }
}
