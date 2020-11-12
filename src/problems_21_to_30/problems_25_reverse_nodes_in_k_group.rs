use crate::problems_21_to_30::ListNode;

/// 25.
///
/// 给你一个链表，每 k 个节点一组进行翻转，请你返回翻转后的链表。
///
/// k 是一个正整数，它的值小于或等于链表的长度。
///
/// 如果节点总数不是 k 的整数倍，那么请将最后剩余的节点保持原有顺序。
///
/// 示例 :
///
/// 给定这个链表：1->2->3->4->5
///
/// 当 k = 2 时，应当返回: 2->1->4->3->5
///
/// 当 k = 3 时，应当返回: 3->2->1->4->5
///
/// 说明 :
///
/// 你的算法只能使用常数的额外空间。
/// 你不能只是单纯的改变节点内部的值，而是需要实际的进行节点交换。
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/reverse-nodes-in-k-group
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut dummy_ref = &mut dummy;
    let mut node_vec = vec![];
    while let Some(v) = dummy_ref {
        let mut section = v.next.take();
        let mut section_ref_mut = &mut section;
        let mut end = false;
        for i in 1..=k {
            if let Some(v2) = section_ref_mut {
                if i == k {
                    v.next = v2.next.take();
                }
                section_ref_mut = &mut v2.next;
            } else {
                end = true;
                break;
            }
        }
        if end {
            v.next = section;
            break;
        } else {
            node_vec.push(section);
            dummy_ref = &mut dummy;
        }
    }
    while let Some(v) = node_vec.pop() {
        let mut node_list = v;
        let mut per = dummy.as_mut().unwrap().next.take();
        while let Some(mut v) = node_list.take() {
            node_list = v.next;
            v.next = per;
            per = Some(v);
        }
        dummy.as_mut().unwrap().next = per;
    }
    dummy.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_k_group() {
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
                            next: None,
                        })),
                    })),
                })),
            })),
        }));
        let res = reverse_k_group(head, 2);
        println!("{:?}", res);
    }
}
