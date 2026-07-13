use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        
        // 1. If the current node exists, peek at it
        if let Some(node) = &root {
            
            // 2. Get mutable access to the inside of the RefCell
            let mut n = node.borrow_mut();
            
            // 3. Rip both child branches completely off the current node
            let left_branch = n.left.take();
            let right_branch = n.right.take();
            
            // 4. Swap them, and recursively invert their children!
            n.left = Self::invert_tree(right_branch);
            n.right = Self::invert_tree(left_branch);
        }
        
        // 5. Return the root (which now has its children permanently swapped)
        root
    }
}