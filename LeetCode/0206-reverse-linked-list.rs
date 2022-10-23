// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
type List = Option<Box<ListNode>>;
impl Solution {
    pub fn reverse_list(l: List) -> List {
        Self::solve(l, None)
    }

    fn solve(l: List, t: List) -> List {
        match l {
            Some(n) => {
                let h = Some(Box::new(ListNode {
                    val: n.val,
                    next: t
                }));
                Self::solve(n.next, h)
            },
            None => t
        }
    }
}
