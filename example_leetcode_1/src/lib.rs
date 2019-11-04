use std::collections::HashMap;
use std::borrow::{Borrow, BorrowMut};
use std::str::Chars;

///
/// 给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
///
/// 你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素。
///
/// 示例:
///
/// 给定 nums = [2, 7, 11, 15], target = 9
///
/// 因为 nums[0] + nums[1] = 2 + 7 = 9
/// 所以返回 [0, 1]
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/two-sum
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm: HashMap<i32, i32> = HashMap::new();
    let mut out: Vec<i32> = vec![];
    for index in 0..nums.len() {
        let diff = target - nums[index];
        if let Some(_x) = hm.get(&diff) {
            out.push(*hm.get(&diff).unwrap());
            out.push(index as i32);
            break;
        }
        hm.insert(nums[index], index as i32);
    }
    out
}

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
///Definition for singly-linked list.
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

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy_head = Some(Box::new(ListNode::new(0)));
    let mut ref_head = &mut dummy_head;
    let mut carry = 0;
    let (mut p1, mut p2) = (&l1, &l2);
    while p1.is_some() || p2.is_some() {
        let mut x = 0;
        let mut y = 0;
        if let Some(v) = p1 {
            x = v.val;
            p1 = v.next.borrow();
        }
        if let Some(v) = p2 {
            y = v.val;
            p2 = v.next.borrow();
        }
        let sum = carry + x + y;
        carry = sum / 10;
        if let Some(v) = ref_head {
            v.next = Some(Box::new(ListNode::new(sum % 10)));
            ref_head = v.next.borrow_mut();
        }
    }
    if carry > 0 {
        if let Some(v) = ref_head {
            v.next = Some(Box::new(ListNode::new(carry)));
        }
    }
    return dummy_head.unwrap().next;
}


///
/// 给定一个字符串，请你找出其中不含有重复字符的 最长子串 的长度。
///
/// 示例 1:
///
/// 输入: "abcabcbb"
/// 输出: 3
/// 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
/// 示例 2:
///
/// 输入: "bbbbb"
/// 输出: 1
/// 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
/// 示例 3:
///
/// 输入: "pwwkew"
/// 输出: 3
/// 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
///     请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/longest-substring-without-repeating-characters
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut ans = 0;
    let mut start = 0;
    let mut map: HashMap<char, usize> = HashMap::new();
    for (index, ch) in s.chars().enumerate() {
        start = start.max(*map.get(ch.borrow()).unwrap_or(&0));
        //判断存储子串长度是否大于检查子串长度，小于检查子串长度获取子串长度
        ans = ans.max(index - start + 1);
        map.insert(ch, index + 1);
    }
    ans as i32
}

///
/// 给定两个大小为 m 和 n 的有序数组 nums1 和 nums2。
///
/// 请你找出这两个有序数组的中位数，并且要求算法的时间复杂度为 O(log(m + n))。
///
/// 你可以假设 nums1 和 nums2 不会同时为空。
///
/// 示例 1:
///
/// nums1 = [1, 3]
/// nums2 = [2]
///
/// 则中位数是 2.0
/// 示例 2:
///
/// nums1 = [1, 2]
/// nums2 = [3, 4]
///
/// 则中位数是 (2 + 3)/2 = 2.5
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/median-of-two-sorted-arrays
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
    let (mut n1_len, mut n2_len) = (nums1.len(), nums2.len());
    if n1_len > n2_len {
        let re = nums1;
        nums1 = nums2;
        nums2 = re;
        let re = n1_len;
        n1_len = n2_len;
        n2_len = re;
    }
    let (mut min, mut max) = (0, n1_len);
    while min <= max {
        let i = (min + max) / 2;
        let j = (n1_len + n2_len + 1) / 2 - i;
        if j != 0 && i != n1_len && (i < max) && (nums2[j - 1] > nums1[i]) {
            min = i + 1;
        } else if i != 0 && j != n2_len && (i > min) && (nums1[i - 1] > nums2[j]) {
            max = i - 1;
        } else {
            let mut max_left: f64 = 0.0;

            if i == 0 {
                max_left = nums2[j - 1] as f64;
            } else if j == 0 {
                max_left = nums1[i - 1] as f64;
            } else {
                max_left = nums1[i - 1].max(nums2[j - 1]) as f64
            }
            if ((n1_len + n2_len) % 2) == 1 {
                return max_left as f64;
            }

            let mut min_right: f64 = 0.0;

            if i == n1_len {
                min_right = nums2[j] as f64;
            } else if j == n2_len {
                min_right = nums1[i] as f64
            } else {
                min_right = nums1[i].min(nums2[j]) as f64;
            }
            return (max_left + min_right) / 2.0;
        }
    }
    0.0
}

#[cfg(test)]
mod test {
    use crate::{two_sum, ListNode, add_two_numbers, length_of_longest_substring};

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(nums, target);

        assert_eq!(result, vec![0, 1])
    }

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

        let result = add_two_numbers(l1, l2);
        assert_eq!(result, Some(Box::new(ListNode { val: 1, next: Some(Box::new(ListNode { val: 8, next: None })) })))
    }

    #[test]
    fn test_length_of_longest_substring() {
        let ans = length_of_longest_substring(String::from("abcabcbb"));
        assert_eq!(ans, 3)
    }
}