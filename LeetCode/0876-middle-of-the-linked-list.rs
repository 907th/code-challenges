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
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p1 = head.clone(); // Full list clone!
        let mut p2 = head;
        loop {
            p2 = p2.unwrap().next;
            if p2.is_none() { break; }
            p2 = p2.unwrap().next;
            p1 = p1.unwrap().next;
            if p2.is_none() { break; }
        }
        p1
    }
}

