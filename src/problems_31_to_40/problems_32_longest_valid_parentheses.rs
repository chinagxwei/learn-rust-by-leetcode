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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_valid_parentheses() {
        let res = longest_valid_parentheses("(()".to_string());
        assert_eq!(res, 2);
        let res = longest_valid_parentheses(")()())".to_string());
        assert_eq!(res, 4);
        let res = longest_valid_parentheses("())".to_string());
        assert_eq!(res, 2);
    }
}
