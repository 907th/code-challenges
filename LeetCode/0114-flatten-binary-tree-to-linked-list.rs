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

type Vertex = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn flatten(root: &mut Option<Vertex>) {
        if root.is_some() {
            let root_rc: Vertex = Rc::clone(root.as_ref().unwrap());
            Self::dfs(root_rc);
        }
    }

    fn dfs(v: Vertex) -> Vertex {
        let is_left = v.borrow().left.is_some();
        let is_right = v.borrow().right.is_some();
        if is_left {
            let l = v.borrow().left.clone().unwrap();
            if is_right {
                let r = v.borrow().right.clone().unwrap();
                {
                    let mut vd = v.borrow_mut();
                    vd.right = Some(Rc::clone(&l));
                    vd.left = None;
                }
                let t = Self::dfs(l);
                {
                    let mut td = t.borrow_mut();
                    td.right = Some(Rc::clone(&r));
                }
                Self::dfs(r)
            } else {
                {
                    let mut vd = v.borrow_mut();
                    vd.right = Some(Rc::clone(&l));
                    vd.left = None;
                }
                Self::dfs(l)
            }
        } else {
            if is_right {
                let r = v.borrow().right.clone().unwrap();
                Self::dfs(r)
            } else {
                v
            }
        }
    }
}
