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
type TreeRef = Rc<RefCell<TreeNode>>;
impl Solution {
    pub fn lowest_common_ancestor(root: Option<TreeRef>, p: Option<TreeRef>, q: Option<TreeRef>) -> Option<TreeRef> {
        let (r, p, q) = (root.unwrap(), p.unwrap(), q.unwrap());
        if Rc::ptr_eq(&r, &p) || Rc::ptr_eq(&r, &q) { return Some(r); }
        let (rv, pv, qv) = (r.borrow().val, p.borrow().val, q.borrow().val);
        if rv > pv && rv > qv {
            Self::lowest_common_ancestor(r.borrow().left.clone(), Some(p), Some(q))
        } else if rv < pv && rv < qv {
            Self::lowest_common_ancestor(r.borrow().right.clone(), Some(p), Some(q))
        } else {
            Some(r)
        }
    }
}
