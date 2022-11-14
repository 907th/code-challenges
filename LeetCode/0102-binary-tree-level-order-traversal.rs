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
use std::collections::VecDeque;

type NodeRef = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn level_order(root: Option<NodeRef>) -> Vec<Vec<i32>> {
        if root.is_none() { return vec![]; }
        let root = root.unwrap();
        let mut ans = Vec::new();
        let mut cur: Vec<i32> = Vec::new();
        let mut q: VecDeque<(NodeRef, i32)> = VecDeque::from(vec![(root, 0)]);
        let mut ll = 0;
        while !q.is_empty() {
            let (v, l) = q.pop_front().unwrap();
            if ll < l {
                ans.push(cur);
                cur = Vec::new();
                ll = l;
            }
            let vv = v.borrow();
            cur.push(vv.val);
            if vv.left.is_some() { q.push_back((Rc::clone(vv.left.as_ref().unwrap()), l + 1)); }
            if vv.right.is_some() { q.push_back((Rc::clone(vv.right.as_ref().unwrap()), l + 1)); }
        }
        if cur.len() > 0 { ans.push(cur); }
        ans
    }
}
