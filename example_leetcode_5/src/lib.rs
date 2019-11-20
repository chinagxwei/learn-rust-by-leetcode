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

/// 21.
///
/// 将两个有序链表合并为一个新的有序链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。 
///
/// 示例：
///
/// 输入：1->2->4, 1->3->4
/// 输出：1->1->2->3->4->4
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/merge-two-sorted-lists
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

/// 递归
pub fn merge_two_lists_1(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn merge_list(list_1: Option<Box<ListNode>>, list_2: Option<Box<ListNode>>, mut head: &mut Box<ListNode>) {
        if list_1 == None {
            head.next = list_2;
        } else if list_2 == None {
            head.next = list_1;
        } else {
            let (mut l1_mut, mut l2_mut) = (list_1, list_2);
            if let (Some(l1_ref), Some(l2_ref)) = (l1_mut.as_ref(), l2_mut.as_ref()) {
                if l1_ref.val < l2_ref.val {
                    head.next = l1_mut;
                    head = head.next.as_mut().unwrap();
                    merge_list(head.next.take(), l2_mut, head);
                } else {
                    head.next = l2_mut;
                    head = head.next.as_mut().unwrap();
                    merge_list(l1_mut, head.next.take(), head);
                }
            }
        }
    }
    let mut dummy = Box::new(ListNode::new(0));
    merge_list(l1, l2, &mut dummy);
    dummy.next
}

/// 遍历
pub fn merge_two_lists_2(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let (mut l1_mut, mut l2_mut, mut head) = (l1, l2, &mut dummy);
    while let (Some(l1_ref), Some(l2_ref)) = (l1_mut.as_ref(), l2_mut.as_ref()) {
        if l1_ref.val <= l2_ref.val {
            head.next = l1_mut;
            head = head.next.as_mut().unwrap();
            l1_mut = head.next.take()
        } else {
            head.next = l2_mut;
            head = head.next.as_mut().unwrap();
            l2_mut = head.next.take();
        }
    }
    head.next = if l1_mut.is_none() { l2_mut } else { l1_mut };
    dummy.next
}

#[cfg(test)]
mod test {
    use crate::{ListNode, merge_two_lists_1, merge_two_lists_2};

    #[test]
    fn test_merge_two_lists() {
        let (l1, l2) = (
            Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode { val: 4, next: None })) })) })),
            Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 3, next: Some(Box::new(ListNode { val: 4, next: None })) })) }))
        );
        let res = merge_two_lists_1(l1, l2);
        println!("{:?}", res);
        let (l1, l2) = (
            Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 3, next: Some(Box::new(ListNode { val: 5, next: None })) })) })),
            Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode { val: 3, next: None })) })) }))
        );
        let res = merge_two_lists_2(l1, l2);
        println!("{:?}", res)
    }
}
