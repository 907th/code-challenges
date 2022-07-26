// Definition for a binary tree node.
//
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

type Vertex = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn lowest_common_ancestor(root: Vertex, p: Vertex, q: Vertex) -> Vertex {
        if let None = root { return None; }
        if let None = p { return q; }
        if let None = q { return p; }
        if p == q { return p; }
        Self::dfs(root, p, q).2
    }

    fn dfs(c: Vertex, p: Vertex, q: Vertex) -> (bool, bool, Vertex) {
        if let None = c { return (false, false, None); }
        let i = (c == p, c == q);
        let cc = c.unwrap();

        let l = Self::dfs(cc.borrow().left.clone(), p.clone(), q.clone());
        if l.0 && l.1 { return l; }

        let r = Self::dfs(cc.borrow().right.clone(), p.clone(), q.clone());
        if r.0 && r.1 { return r; }

        if (i.0 || l.0 || r.0) && (i.1 || l.1 || r.1) {
            (true, true, Some(cc))
        } else {
            (i.0 || l.0 || r.0, i.1 || l.1 || r.1, None)
        }
    }
}
