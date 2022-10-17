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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut k = 0;
        let mut p = &head;
        while p.is_some() {
            p = &p.as_ref().unwrap().next;
            k += 1;
        }

        if n == k { return head.unwrap().next; }

        let mut i = 0;
        let mut h = head.clone(); // Full clone of the list!
        let mut p = &mut h;
        loop {
            i += 1;
            if i == k - n {
                let r = p.as_mut().unwrap();
                r.next = r.next.as_ref().unwrap().next.clone(); // Full clone of the tail of the list!
                break;
            }
            p = &mut p.as_mut().unwrap().next;
        }
        h
    }
}

