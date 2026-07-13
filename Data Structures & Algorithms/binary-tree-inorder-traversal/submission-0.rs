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

    pub fn dfs(root: Option<&RefCell<TreeNode>>, v: &mut Vec<i32>){
        if let Some(node) = root{
            Self::dfs(node.borrow().left.as_deref(), v);
            v.push(node.borrow().val);
            Self::dfs(node.borrow().right.as_deref(),v);
        }

    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = Vec::new();

        Self::dfs(root.as_deref(), &mut v);
        v
    }
}
