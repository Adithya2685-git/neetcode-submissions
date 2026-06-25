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

    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, depth:i32, highest: &mut i32){
        if let Some(node) = root{
            Self::dfs(node.borrow().left.clone(), depth+1, highest);
            Self::dfs(node.borrow().right.clone(), depth+1, highest);
            *highest= (*highest).max(depth)
        }
    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let depth =1;
        let mut highest =0;
        Self::dfs(root, depth, &mut highest);
        highest
    }
}
