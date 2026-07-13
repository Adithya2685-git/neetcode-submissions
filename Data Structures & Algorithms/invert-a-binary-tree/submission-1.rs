use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    // Your dedicated helper function, taking a reference via as_deref()
    pub fn dfs(root: Option<&RefCell<TreeNode>>) {
        if let Some(node) = root {
            
            // 1. Get mutable access to the current node
            let mut n = node.borrow_mut();
            
            // 2. Swap the left and right children in place
            let temp = n.left.take();
            n.left = n.right.take();
            n.right = temp;

            // 3. Recurse down the tree using .as_deref()
            Self::dfs(n.left.as_deref());
            Self::dfs(n.right.as_deref());
        }
    }

    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        
        // Use .as_deref() to pass a reference to the DFS function. 
        // This strips the Rc wrapper but DOES NOT consume the tree!
        Self::dfs(root.as_deref());
        
        // Because we only passed a reference, we still own 'root' and can return it
        root
    }
}