/// 31.
///
/// 实现获取下一个排列的函数，算法需要将给定数字序列重新排列成字典序中下一个更大的排列。
///
/// 如果不存在下一个更大的排列，则将数字重新排列成最小的排列（即升序排列）。
///
/// 必须原地修改，只允许使用额外常数空间。
///
/// 以下是一些例子，输入位于左侧列，其相应输出位于右侧列。
/// 1,2,3 → 1,3,2
/// 3,2,1 → 1,2,3
/// 1,1,5 → 1,5,1
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/next-permutation
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///
pub fn next_permutation(nums: &mut Vec<i32>) {
    let (mut i, nums_len) = ((nums.len() - 2) as i32, nums.len());
    let swap = |tmp_nums: &mut Vec<i32>, i: usize, j: usize| {
        let tmp = tmp_nums[i];
        tmp_nums[i] = tmp_nums[j];
        tmp_nums[j] = tmp;
    };
    while i >= 0 && nums[(i + 1) as usize] <= nums[i as usize] {
        i -= 1;
    }
    if i >= 0 {
        let mut j = (nums_len - 1) as i32;
        while j >= 0 && nums[j as usize] <= nums[i as usize] {
            j -= 1
        }
        swap(nums, i as usize, j as usize);
    }
    let (mut start, mut end) = ((i + 1) as usize, nums_len - 1);
    while start < end {
        swap(nums, start, end);
        start += 1;
        end -= 1;
    }
}

/// 32.
///
/// 给定一个只包含 '(' 和 ')' 的字符串，找出最长的包含有效括号的子串的长度。
///
/// 示例 1:
///
/// 输入: "(()"
/// 输出: 2
/// 解释: 最长有效括号子串为 "()"
/// 示例 2:
///
/// 输入: ")()())"
/// 输出: 4
/// 解释: 最长有效括号子串为 "()()"
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/longest-valid-parentheses
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///
pub fn longest_valid_parentheses(s: String) -> i32 {
    let mut ans = 0;
    let mut dp = vec![0; s.len()];
    let s_byte = s.as_bytes();
    for i in 1..s.len() {
        if s_byte[i] == b')' {
            if s_byte[i - 1] == b'(' {
                dp[i] = if i >= 2 { dp[i - 2] } else { 0 } + 2;
            } else if (i as i32 - dp[i - 1] as i32 - 1) >= 0 {
                if (i - dp[i] as usize) > 0 && s_byte[i - dp[i - 1] as usize - 1] == b'(' {
                    dp[i] = dp[i - 1] + if (i - dp[i - 1] as usize) >= 2 { dp[i - dp[i - 1] as usize - 2] } else { 0 } + 2;
                }
            }
            ans = ans.max(dp[i]);
        }
    }
    ans
}

/// 33.
///
/// 假设按照升序排序的数组在预先未知的某个点上进行了旋转。
///
/// ( 例如，数组 [0,1,2,4,5,6,7] 可能变为 [4,5,6,7,0,1,2] )。
///
/// 搜索一个给定的目标值，如果数组中存在这个目标值，则返回它的索引，否则返回 -1 。
///
/// 你可以假设数组中不存在重复的元素。
///
/// 你的算法时间复杂度必须是 O(log n) 级别。
///
/// 示例 1:
///
/// 输入: nums = [4,5,6,7,0,1,2], target = 0
/// 输出: 4
/// 示例 2:
///
/// 输入: nums = [4,5,6,7,0,1,2], target = 3
/// 输出: -1
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/search-in-rotated-sorted-array
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn search_1(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, (nums.len() - 1) as i32);
    while left <= right {
        let middle = (left + right) / 2;
        let mut num = nums[middle as usize];
        if (nums[middle as usize] < nums[0]) == (target < nums[0]) {
            num = nums[middle as usize];
        } else {
            num = if target < nums[0] { std::i32::MIN } else { std::i32::MAX }
        }
        if num < target {
            left = middle + 1;
        } else if num > target {
            right = middle - 1;
        } else {
            return middle;
        }
    }
    -1
}

pub fn search_2(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, (nums.len() - 1) as i32);
    while left <= right {
        let middle = (left + right) / 2;
        if nums[left as usize] <= nums[middle as usize] {
            if nums[left as usize] <= target && target < nums[middle as usize] {
                right = middle - 1;
            } else {
                left = middle + 1;
            }
        } else {
            if nums[middle as usize] < target && target <= nums[right as usize] {
                left = middle + 1;
            } else {
                right = middle - 1;
            }
        }
    }
    -1
}

/// 34.
///
/// 给定一个按照升序排列的整数数组 nums，和一个目标值 target。找出给定目标值在数组中的开始位置和结束位置。
///
/// 你的算法时间复杂度必须是 O(log n) 级别。
///
/// 如果数组中不存在目标值，返回 [-1, -1]。
///
/// 示例 1:
///
/// 输入: nums = [5,7,7,8,8,10], target = 8
/// 输出: [3,4]
/// 示例 2:
///
/// 输入: nums = [5,7,7,8,8,10], target = 6
/// 输出: [-1,-1]
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/find-first-and-last-position-of-element-in-sorted-array
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() == 0 { return vec![-1, -1]; }
    let (mut left, mut right, mut result) = (0_i32, (nums.len() - 1) as i32, vec![]);
    while left < right {
        let middle = (left + right) / 2;
        if target <= nums[middle as usize] {
            right = middle;
        } else {
            left = middle + 1;
        }
    }
    result.push(if target == nums[left as usize] { left } else { -1 });
    right = (nums.len() - 1) as i32;
    while left < right {
        let middle = (left + right + 1) / 2;
        if target == nums[middle as usize] {
            left = middle;
        } else {
            right = middle - 1;
        }
    }
    result.push(if target == nums[right as usize] { right } else { -1 });
    result
}

/// 35.
///
/// 给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。
///
/// 你可以假设数组中无重复元素。
///
/// 示例 1:
///
/// 输入: [1,3,5,6], 5
/// 输出: 2
/// 示例 2:
///
/// 输入: [1,3,5,6], 2
/// 输出: 1
/// 示例 3:
///
/// 输入: [1,3,5,6], 7
/// 输出: 4
/// 示例 4:
///
/// 输入: [1,3,5,6], 0
/// 输出: 0
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/search-insert-position
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

/// 暴力法
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    for (i, num) in nums.iter().enumerate() {
        if *num == target {
            return i as i32;
        } else if *num > target {
            return i as i32;
        }
    }
    nums.len() as i32
}

/// 二分法
pub fn search_insert_2(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, (nums.len() - 1) as i32);
    while left <= right {
        let middle = (left + right) / 2;
        if nums[middle as usize] == target {
            return middle;
        } else if nums[middle as usize] < target {
            left = middle + 1;
        } else {
            right = middle - 1;
        }
    }
    return left;
}


#[cfg(test)]
mod test {
    use crate::{next_permutation, longest_valid_parentheses, search_1, search_range, search_insert, search_insert_2, search_2};

    #[test]
    fn test_next_permutation() {
        let mut input = vec![1, 2, 3];
        next_permutation(&mut input);
        assert_eq!(input, [1, 3, 2]);
        let mut input = vec![3, 2, 1];
        next_permutation(&mut input);
        assert_eq!(input, [1, 2, 3]);
        let mut input = vec![1, 1, 5];
        next_permutation(&mut input);
        assert_eq!(input, [1, 5, 1]);
    }

    #[test]
    fn test_longest_valid_parentheses() {
        let res = longest_valid_parentheses("(()".to_string());
        assert_eq!(res, 2);
        let res = longest_valid_parentheses(")()())".to_string());
        assert_eq!(res, 4);
        let res = longest_valid_parentheses("())".to_string());
        assert_eq!(res, 2);
    }

    #[test]
    fn test_search() {
        let res = search_1(vec![4, 5, 6, 7, 0, 1, 2], 0);
        assert_eq!(res, 4);
        let res = search_2(vec![4, 5, 6, 7, 0, 1, 2], 3);
        assert_eq!(res, -1);
    }

    #[test]
    fn test_search_range() {
        let res = search_range(vec![5, 7, 7, 8, 8, 10], 8);
        assert_eq!(res, [3, 4]);
        let res = search_range(vec![5, 7, 7, 8, 8, 10], 6);
        assert_eq!(res, [-1, -1]);
    }

    #[test]
    fn test_search_insert() {
        let res = search_insert(vec![1, 3, 5, 6], 4);
        assert_eq!(res, 2);
        let res = search_insert(vec![1, 3, 5, 6], 2);
        assert_eq!(res, 1);
    }
}
