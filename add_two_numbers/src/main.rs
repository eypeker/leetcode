use std::ops;

fn main() {
    println!("Hello, world!");
}
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
impl Default for ListNode{
    fn default() -> Self {
        Self { val: 0, next: None }
    }
}
impl ops::Add<i32> for ListNode{
    type Output = ListNode;

    fn add(self, rhs: i32) -> Self::Output {
        ListNode{val: self.val + rhs, next: self.next}
    }
}



// impl ops::Add<i32> for Option<ListNode>{
//     type Output=Option<ListNode>;

//     fn add(self, rhs: i32) -> Self::Output {
//         match self {
//             None => Some(ListNode { val: rhs, next: None }),
//             Some(me) => Some(ListNode { val:me.val , next: me.next })
//         }
//     }
// } 

pub struct Solution;


impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let (mut iter1,mut  iter2) = match (l1,l2){
            (None, None) => return None,
            (None, Some(y)) => (ListNode{val: 0, next: None}, *y),
            (Some(x),None) => (*x, ListNode{val: 0, next: None}),
            (Some(a), Some(b)) => (*a, *b)
        };
        let val = iter1.val + iter2.val;
        let small = val %10;
        let remainder = val /10;
        if remainder > 0 {
            iter1.next = match iter1.next{
            None => Some(Box::new(ListNode{ val:remainder, next:None})),
            Some(mut x) => {x.val = x.val + remainder; Some(x)}
            };
        }
        

        Some(Box::new(ListNode{val:small, next: Solution::add_two_numbers(iter1.next, iter2.next)}))

    }
}