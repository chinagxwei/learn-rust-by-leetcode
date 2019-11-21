use std::borrow::BorrowMut;

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

/// 22.
///
/// 给出 n 代表生成括号的对数，请你写出一个函数，使其能够生成所有可能的并且有效的括号组合。
///
/// 例如，给出 n = 3，生成结果为：
///
/// [
///   "((()))",
///   "(()())",
///   "(())()",
///   "()(())",
///   "()()()"
/// ]
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/generate-parentheses
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

/// 回溯法
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn backtrack(ans: &mut Vec<String>, current: &str, open: i32, close: i32, max: i32) {
        if current.len() == (max * 2) as usize {
            ans.push(current.to_string())
        } else {
            if open < max {
                let mut cur = current.to_string();
                cur.push_str("(");
                backtrack(ans, &cur, open + 1, close, max)
            }
            if close < open {
                let mut cur = current.to_string();
                cur.push_str(")");
                backtrack(ans, &cur, open, close + 1, max)
            }
        }
    }
    let mut ans = vec![];
    backtrack(&mut ans, "", 0, 0, n);
    ans
}

/// 23.
///
/// 合并 k 个排序链表，返回合并后的排序链表。请分析和描述算法的复杂度。
///
/// 示例:
///
/// 输入:
/// [
///   1->4->5,
///   1->3->4,
///   2->6
/// ]
/// 输出: 1->1->2->3->4->4->5->6
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/merge-k-sorted-lists
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

///使用两两合并合并列表
pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    while lists.len() > 1 {
        let mut tmp_lists = vec![];
        while let Some(l1) = lists.pop() {
            if let Some(l2) = lists.pop() {
                tmp_lists.push(merge_two_lists_2(l1, l2));
            } else {
                tmp_lists.push(l1);
            }
        }
        lists = tmp_lists;
    }
    lists.pop().unwrap_or(None)
}

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
    let mut point = &mut dummy;
    while point.as_ref().unwrap().next.is_some() && point.as_ref().unwrap().next.as_ref().unwrap().next.is_some() {
        let mut left = point.as_mut().unwrap().next.take();
        let mut right = left.as_mut().unwrap().next.take();
        left.as_mut().unwrap().next = right.as_mut().unwrap().next.take();
        right.as_mut().unwrap().next = left;
        point.as_mut().unwrap().next = right;
        point = &mut point.as_mut().unwrap().next.as_mut().unwrap().next;
    }
    dummy.unwrap().next
}

#[cfg(test)]
mod test {
    use crate::{ListNode, merge_two_lists_1, merge_two_lists_2, generate_parenthesis, merge_k_lists, swap_pairs};

    #[test]
    fn test_merge_two_lists() {
        let (l1, l2) = (
            Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 2, next: None })) })),
            Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode { val: 3, next: None })) }))
        );
        let res = merge_two_lists_1(l1, l2);
        assert_eq!(
            res,
            Some(Box::new(ListNode {
                val: 1,
                next:
                Some(Box::new(ListNode {
                    val: 2,
                    next:
                    Some(Box::new(ListNode {
                        val: 2,
                        next:
                        Some(Box::new(ListNode { val: 3, next: None })),
                    })),
                })),
            }))
        );
        let (l1, l2) = (
            Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 3, next: None })) })),
            Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 2, next: None })) }))
        );
        let res = merge_two_lists_2(l1, l2);
        assert_eq!(
            res,
            Some(Box::new(ListNode {
                val: 1,
                next:
                Some(Box::new(ListNode {
                    val: 1,
                    next:
                    Some(Box::new(ListNode {
                        val: 2,
                        next:
                        Some(Box::new(ListNode { val: 3, next: None })),
                    })),
                })),
            }))
        );
    }

    #[test]
    fn test_generate_parenthesis() {
        let res = generate_parenthesis(2);
        println!("{:?}", res);
        assert_eq!(res, ["(())", "()()"])
    }

    #[test]
    fn test_merge_k_lists() {
        let lists = vec![
            Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 4, next: Some(Box::new(ListNode { val: 5, next: None })) })) })),
            Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 3, next: Some(Box::new(ListNode { val: 4, next: None })) })) })),
            Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode { val: 6, next: None })) }))
        ];

        let res = merge_k_lists(lists);
        println!("{:?}", res);
    }

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
