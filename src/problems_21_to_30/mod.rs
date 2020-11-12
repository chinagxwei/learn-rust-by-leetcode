mod problems_21_merge_two_sorted_lists;
mod problems_22_generate_parentheses;
mod problems_23_merge_k_sorted_lists;
mod problems_24_swap_nodes_in_pairs;
mod problems_25_reverse_nodes_in_k_group;
mod problems_26_remove_duplicates_from_sorted_array;
mod problems_27_remove_element;
mod problems_28_implement_strstr;
mod problems_29_divide_two_integers;
mod problems_30_substring_with_concatenation_of_all_words;

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
