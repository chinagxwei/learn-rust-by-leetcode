/// 19.
///
/// 给定一个链表，删除链表的倒数第 n 个节点，并且返回链表的头结点。
///
/// 示例：
///
/// 给定一个链表: 1->2->3->4->5, 和 n = 2.
///
/// 当删除了倒数第二个节点后，链表变为 1->2->3->5.
/// 说明：
///
/// 给定的 n 保证是有效的。
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

/// 两次遍历
pub fn remove_nth_from_end_1(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut find = &dummy.as_ref().unwrap().next;
    let mut len = 0;
    while let Some(v) = find {
        find = &v.next;
        len += 1;
    }
    len = len - n;
    let mut edit = &mut dummy;
    while len > 0 {
        if let Some(v) = edit {
            edit = &mut v.next
        }
        len -= 1;
    }
    if let Some(v) = edit {
        v.next = v.next.as_mut().unwrap().next.take();
    }
    dummy.unwrap().next
}

/// 一次遍历
pub fn remove_nth_from_end_2(head: Option<Box<ListNode>>, mut n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let first = {
        let mut tmp = &dummy.as_ref().unwrap().next;
        while n != 0 {
            if let Some(v) = tmp {
                tmp = &(v.next)
            }
            n -= 1;
        }
        tmp.clone()
    };
    let (mut f, mut s) = (&first, &mut dummy);
    while let Some(v) = f {
        f = &v.next;
        if let Some(y) = s {
            s = &mut y.next
        }
    }
    if let Some(v) = s {
        v.next = v.next.as_mut().unwrap().next.take();
    }
    dummy.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_nth_from_end() {
        let head =
            Some(Box::new(ListNode {
                val: 1,
                next: None,
            }
            ));
        let res = remove_nth_from_end_1(head, 1);
        assert_eq!(res, None);
        let head =
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: None,
                }
                )),
            }
            ));
        let res = remove_nth_from_end_2(head, 2);
        assert_eq!(
            res,
            Some(Box::new(ListNode { val: 2, next: None }))
        );
    }
}
