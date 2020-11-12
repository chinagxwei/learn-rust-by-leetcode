use crate::problems_21_to_30::{ListNode, merge_two_lists_2};

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
