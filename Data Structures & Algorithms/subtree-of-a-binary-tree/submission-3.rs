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
    pub fn is_same(root:Option<&RefCell<TreeNode>>, sub_head: Option<&RefCell<TreeNode>>)->bool {
        if root.is_some() && sub_head.is_some(){
            let n1 = root.unwrap();
            let n2 = sub_head.unwrap();
            let v1 = n1.borrow().val;
            let v2 = n2.borrow().val;

            if v1 != v2{
                return false;
            }

            let left = Self::is_same(n1.borrow().left.as_deref(), n2.borrow().left.as_deref());
            let right =Self::is_same(n1.borrow().right.as_deref(), n2.borrow().right.as_deref());
            
            left && right
        }else if !root.is_some() && !sub_head.is_some(){
            true
        }else{
            false
        }

    }
    pub fn dfs(root: Option<&RefCell<TreeNode>>, sub_head: Option<&RefCell<TreeNode>>)-> bool{
        
        if root.is_some(){
            let n1 = root.unwrap();

            if Self::is_same(root, sub_head){
                return true;
            }

            let left =Self::dfs(n1.borrow().left.as_deref(),sub_head);
            let right =Self::dfs(n1.borrow().right.as_deref(),sub_head);                
            return left || right

        }else{
            false
        }
    }

    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {

        Self::dfs(root.as_deref(),sub_root.as_deref())        
    }
}
