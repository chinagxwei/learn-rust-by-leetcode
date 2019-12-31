use std::collections::HashMap;

/// 3.
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

struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut ans, mut start) = (0, 0);
        let mut map = HashMap::<char, usize>::new();
        for (index, ch) in s.chars().enumerate() {
            start = start.max(*map.get(&ch).unwrap_or(&0));
            //判断存储子串长度是否大于检查子串长度，小于检查子串长度获取子串长度
            ans = ans.max(index - start + 1);
            map.insert(ch, index + 1);
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        let ans = Solution::length_of_longest_substring(String::from("abcabcbb"));
        assert_eq!(ans, 3)
    }
}