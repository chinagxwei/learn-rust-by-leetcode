/// 2.
///
/// 给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的，并且它们的每个节点只能存储 一位 数字。
///
/// 如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。
///
/// 您可以假设除了数字 0 之外，这两个数都不会以 0 开头。
///
/// 示例：
///
/// 输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
/// 输出：7 -> 0 -> 8
/// 原因：342 + 465 = 807
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/add-two-numbers
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///
/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut head_ref = &mut dummy_head;
        let mut carry = 0;
        let (mut l1_ref, mut l2_ref) = (&l1, &l2);
        while l1_ref.is_some() || l2_ref.is_some() {
            let (mut x, mut y) = (0, 0);
            if let Some(v) = l1_ref {
                x = v.val;
                l1_ref = &v.next;
            }
            if let Some(v) = l2_ref {
                y = v.val;
                l2_ref = &v.next;
            }
            let sum = carry + x + y;
            carry = sum / 10;
            if let Some(v) = head_ref {
                v.next = Some(Box::new(ListNode::new(sum % 10)));
                head_ref = &mut v.next;
            }
        }
        if carry > 0 {
            if let Some(v) = head_ref {
                v.next = Some(Box::new(ListNode::new(carry)));
            }
        }
        return dummy_head.unwrap().next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let l1 = Some(
            Box::new(
                ListNode {
                    val: 1,
                    next: Some(Box::new(
                        ListNode {
                            val: 8,
                            next: None,
                        }
                    )),
                }
            )
        );

        let l2 = Some(
            Box::new(
                ListNode {
                    val: 0,
                    next: None,
                }
            )
        );

        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(result, Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 8, next: None })) })))
    }
}
