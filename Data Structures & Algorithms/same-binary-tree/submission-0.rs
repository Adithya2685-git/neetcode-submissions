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
    pub fn dfs(p: Option<&RefCell<TreeNode>> , q: Option<&RefCell<TreeNode>>)->bool{
        if p.is_some() && q.is_some(){
            let n1 = p.unwrap();
            let n2 = q.unwrap();

            let v1 = n1.borrow().val;
            let v2 = n2.borrow().val;

            if v1 == v2{
                let left= Self::dfs(n1.borrow().left.as_deref(), n2.borrow().left.as_deref());
                let right=Self::dfs(n1.borrow().right.as_deref(), n2.borrow().right.as_deref());

                return left && right
            }else{
                return false;
            }

        }else if !p.is_some() && !q.is_some(){
            true
        }else{ 
            false
        }
    }
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(p.as_deref(), q.as_deref())
    }
}
