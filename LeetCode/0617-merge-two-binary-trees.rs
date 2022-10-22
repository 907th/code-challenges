// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
type X = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn merge_trees(r1: X, r2: X) -> X {
        if r1.is_none() && r2.is_none() { return None; }
        let (v1, l1, r1) =
            if let Some(rc) = r1 {
                let t = rc.borrow();
                (t.val, t.left.clone(), t.right.clone())    
            } else {
                (0, None, None)
            };
        let (v2, l2, r2) =
            if let Some(rc) = r2 {
                let t = rc.borrow();
                (t.val, t.left.clone(), t.right.clone())    
            } else {
                (0, None, None)
            };
        Some(Rc::new(RefCell::new(
            TreeNode {
                val: v1 + v2,
                left: Self::merge_trees(l1, l2),
                right: Self::merge_trees(r1, r2)
            }
        )))
    }
}
