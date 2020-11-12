use crate::problems_21_to_30::ListNode;

/// 24.
///
/// 给定一个链表，两两交换其中相邻的节点，并返回交换后的链表。
///
/// 你不能只是单纯的改变节点内部的值，而是需要实际的进行节点交换。
///
///  
///
/// 示例:
///
/// 给定 1->2->3->4, 你应该返回 2->1->4->3.
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/swap-nodes-in-pairs
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut pointer = &mut dummy;
    while pointer.as_ref().unwrap().next.is_some() && pointer.as_ref().unwrap().next.as_ref().unwrap().next.is_some() {
        let mut left = pointer.as_mut().unwrap().next.take();
        let mut right = left.as_mut().unwrap().next.take();
        left.as_mut().unwrap().next = right.as_mut().unwrap().next.take();
        right.as_mut().unwrap().next = left;
        pointer.as_mut().unwrap().next = right;
        pointer = &mut pointer.as_mut().unwrap().next.as_mut().unwrap().next;
    }
    dummy.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_pairs() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next:
            Some(Box::new(ListNode {
                val: 2,
                next:
                Some(Box::new(ListNode {
                    val: 3,
                    next:
                    Some(Box::new(ListNode {
                        val: 4,
                        next:
                        Some(Box::new(ListNode {
                            val: 5,
                            next:
                            Some(Box::new(ListNode { val: 6, next: None })),
                        })),
                    })),
                })),
            })),
        }));
        let res = swap_pairs(head);
        println!("{:?}", res);
    }
}
