use crate::Solution;

// 
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

// impl Solution {
//     pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
//     }

//     pub fn len(l: Option<Box<ListNode>>) -> usize{
//         let mut size = 0_usize;
//         while let Some(n) = l {
//             size += 1;
//             l = n.next;
//         }

//         size
//     }
// }