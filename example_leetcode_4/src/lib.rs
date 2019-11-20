/// 16.
///
/// 给定一个包括 n 个整数的数组 nums 和 一个目标值 target。找出 nums 中的三个整数，使得它们的和与 target 最接近。返回这三个数的和。假定每组输入只存在唯一答案。
///
/// 例如，给定数组 nums = [-1，2，1，-4], 和 target = 1.
///
/// 与 target 最接近的三个数的和为 2. (-1 + 2 + 1 = 2).
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/3sum-closest
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    let nums_len = nums.len();
    if nums_len < 3 { return 0; }
    nums.sort_unstable();
    let mut ans = nums[0] + nums[1] + nums[2];
    for i in 0..nums_len {
        let (mut left, mut right) = (i + 1, nums_len - 1);
        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            if (target - sum).abs() < (target - ans).abs() {
                ans = sum;
            }
            if sum > target {
                right -= 1;
            } else if sum < target {
                left += 1;
            } else {
                return ans;
            }
        }
    }
    ans
}

/// 17.
///
/// 给定一个仅包含数字 2-9 的字符串，返回所有它能表示的字母组合。
///
/// 给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。
///
///
///
/// 示例:
///
/// 输入："23"
/// 输出：["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
/// 说明:
/// 尽管上面的答案是按字典序排列的，但是你可以任意选择答案输出的顺序。
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/letter-combinations-of-a-phone-number
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn letter_combinations(digits: String) -> Vec<String> {
    let mut res_vec = vec![];
    if digits.len() <= 0 { return res_vec; }
    let get_keyboard_val = |num: &str| -> Option<&str> {
        match num {
            "2" => Some("abc"),
            "3" => Some("def"),
            "4" => Some("ghi"),
            "5" => Some("jkl"),
            "6" => Some("mno"),
            "7" => Some("pqrs"),
            "8" => Some("tuv"),
            "9" => Some("wxyz"),
            _ => None
        }
    };
    for ch in get_keyboard_val(digits.get(0..=0).unwrap()).unwrap().chars() {
        res_vec.push(ch.to_string());
    }
    for i in 1..digits.len() {
        let mut len = res_vec.len();
        while len > 0 {
            for ch in get_keyboard_val(digits.get(i..=i).unwrap()).unwrap().chars() {
                let mut s = res_vec.get(0).unwrap().clone();
                s.push(ch);
                res_vec.push(s);
            }
            res_vec.remove(0);
            len -= 1;
        }
    }
    res_vec
}

/// 18.
///
/// 给定一个包含 n 个整数的数组 nums 和一个目标值 target，判断 nums 中是否存在四个元素 a，b，c 和 d ，
/// 使得 a + b + c + d 的值与 target 相等？找出所有满足条件且不重复的四元组。
///
/// 注意：
///
/// 答案中不可以包含重复的四元组。
///
/// 示例：
///
/// 给定数组 nums = [1, 0, -1, 0, -2, 2]，和 target = 0。
///
/// 满足要求的四元组集合为：
/// [
///   [-1,  0, 0, 1],
///   [-2, -1, 1, 2],
///   [-2,  0, 0, 2]
/// ]
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/4sum
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let nums_len = nums.len();
    if nums_len < 4 { return vec![]; }
    nums.sort_unstable();
    ///
    /// 递归内部函数
    ///
    fn x_sum(x_nums: &Vec<i32>, stack: &mut Vec<i32>, stack_index: i32, k: usize, begin: i32, x_target: i32) -> Vec<Vec<i32>> {
        let mut x_ans = vec![];
        if k == 2 {
            let (mut left, mut right) = (begin, x_nums.len() - 1);
            while left < right as i32 {
                let mut tmp_ans = vec![];
                if x_nums[left as usize] + x_nums[right as usize] > x_target {
                    right -= 1;
                } else if x_nums[left as usize] + x_nums[right as usize] < x_target {
                    left += 1;
                } else {
                    stack[(stack_index + 1) as usize] = x_nums[left as usize];
                    stack[(stack_index + 2) as usize] = x_nums[right as usize];
                    for i in 0..=(stack_index + 2) {
                        tmp_ans.push(stack[i as usize]);
                    }
                    x_ans.push(tmp_ans);
                    right -= 1;
                    left += 1;
                    while left < right as i32 && x_nums[left as usize] == x_nums[left as usize - 1] {
                        left += 1;
                    }
                    while right as i32 > left && right < x_nums.len() - 1 && x_nums[right as usize] == x_nums[right as usize + 1] {
                        right -= 1;
                    }
                }
            }
        } else {
            for i in begin..(x_nums.len() - k + 1) as i32 {
                if i > begin && x_nums[i as usize] == x_nums[i as usize - 1] {
                    continue;
                }
                stack[(stack_index + 1) as usize] = x_nums[i as usize];
                let tmpe_ans = x_sum(x_nums, stack, stack_index + 1, k - 1, i + 1, x_target - x_nums[i as usize]);
                if tmpe_ans.len() <= 0 { continue; }
                x_ans.extend_from_slice(&tmpe_ans);
            }
        }
        x_ans
    }
    let k = 4;
    let mut current_stack = vec![0_i32; k];
    let stack_index = -1;
    let begin = 0;
    x_sum(&nums, &mut current_stack, stack_index, k, begin, target)
}

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

/// 20.
///
/// 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串，判断字符串是否有效。
///
/// 有效字符串需满足：
///
/// 左括号必须用相同类型的右括号闭合。
/// 左括号必须以正确的顺序闭合。
/// 注意空字符串可被认为是有效字符串。
///
/// 示例 1:
///
/// 输入: "()"
/// 输出: true
/// 示例 2:
///
/// 输入: "()[]{}"
/// 输出: true
/// 示例 3:
///
/// 输入: "(]"
/// 输出: false
/// 示例 4:
///
/// 输入: "([)]"
/// 输出: false
/// 示例 5:
///
/// 输入: "{[]}"
/// 输出: true
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/valid-parentheses
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn is_valid(s: String) -> bool {
    if s.len() % 2 != 0 { return false; }
    let get_brackets_val = |num: char| -> Option<char> {
        match num {
            '(' => Some(')'),
            '[' => Some(']'),
            '{' => Some('}'),
            _ => Some('?')
        }
    };
    let (left, mut stack) = ("([{", vec![]);
    for ch in s.chars() {
        if left.find(ch).is_some() {
            stack.push(ch);
        } else {
            if get_brackets_val(stack.pop().unwrap_or('?')).unwrap() != ch {
                return false;
            }
        }
    }
    stack.len() == 0
}

#[cfg(test)]
mod test {
    use crate::{three_sum_closest, letter_combinations, four_sum, ListNode, remove_nth_from_end_1, remove_nth_from_end_2, is_valid};

    #[test]
    fn test_three_sum_closest() {
        let res = three_sum_closest([-1, 2, 1, -4].to_vec(), 1);
        assert_eq!(res, 2)
    }

    #[test]
    fn test_letter_combinations() {
        let res = letter_combinations("23".to_string());
        assert_eq!(res, ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);
        let res = letter_combinations("".to_string());
        assert_eq!(res.len(), 0)
    }

    #[test]
    fn test_four_sum() {
        let res = four_sum([1, 0, -1, 0, -2, 2].to_vec(), 0);
        assert_eq!(res, [[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]])
    }

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

    #[test]
    fn test_is_valid() {
        let res = is_valid("()".to_string());
        assert!(res);
        let res = is_valid("()[]{}".to_string());
        assert!(res);
        let res = is_valid("{[]}".to_string());
        assert!(res);
        let res = is_valid("(]".to_string());
        assert!(!res);
        let res = is_valid("([)]".to_string());
        assert!(!res);
        let res = is_valid("]".to_string());
        assert!(!res);
        let res = is_valid("){".to_string());
        assert!(!res);
    }
}
