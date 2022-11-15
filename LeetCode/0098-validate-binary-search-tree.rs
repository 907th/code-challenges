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
use std::cmp::{min, max};
type TreeRef = Rc<RefCell<TreeNode>>;
impl Solution {
    pub fn is_valid_bst(root: Option<TreeRef>) -> bool {
        Self::check_bst(root).unwrap().0
    }

    fn check_bst(v: Option<TreeRef>) -> Option<(bool, i32, i32)> {
        match v {
            Some(ref rc) => {
                let node = rc.borrow();
                let (val, left, right) = (node.val, node.left.clone(), node.right.clone());
                let (mut self_ok, mut self_min, mut self_max) = (true, val, val);
                if let Some((left_ok, left_min, left_max)) = Self::check_bst(left) {
                    self_ok = self_ok && left_ok && left_max < val;
                    self_min = min(self_min, left_min);
                    self_max = max(self_max, left_max);
                }
                if let Some((right_ok, right_min, right_max)) = Self::check_bst(right) {
                    self_ok = self_ok && right_ok && right_min > val;
                    self_min = min(self_min, right_min);
                    self_max = max(self_max, right_max);
                }
                Some((self_ok, self_min, self_max))
            },
            None => None
        }
    }
}
