/// 6.
///
/// 将一个给定字符串根据给定的行数，以从上往下、从左到右进行 Z 字形排列。
///
/// 比如输入字符串为 "LEETCODEISHIRING" 行数为 3 时，排列如下：
///
/// L   C   I   R
/// E T O E S I I G
/// E   D   H   N
/// 之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："LCIRETOESIIGEDHN"。
///
/// 请你实现这个将字符串进行指定行数变换的函数：
///
/// string convert(string s, int numRows);
/// 示例 1:
///
/// 输入: s = "LEETCODEISHIRING", numRows = 3
/// 输出: "LCIRETOESIIGEDHN"
/// 示例 2:
///
/// 输入: s = "LEETCODEISHIRING", numRows = 4
/// 输出: "LDREOEIIECIHNTSG"
/// 解释:
///
/// L     D     R
/// E   O E   I I
/// E C   I H   N
/// T     S     G
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/zigzag-conversion
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 { return s; }
    let take = num_rows.min(s.len() as i32) as usize;
    let mut rows = vec![String::new(); take];
    let (mut cur_row, mut going_down, mut ret) = (0_i32, false, String::new());
    for ch in s.chars() {
        rows[cur_row as usize].push(ch);
        if cur_row == 0 || cur_row == (num_rows - 1) { going_down = !going_down }
        if going_down { cur_row += 1 } else { cur_row -= 1 };
    }
    for row in rows {
        ret.push_str(&row)
    }
    ret
}

/// 7.
///
/// 给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。
///
/// 示例 1:
///
/// 输入: 123
/// 输出: 321
/// 示例 2:
///
/// 输入: -123
/// 输出: -321
/// 示例 3:
///
/// 输入: 120
/// 输出: 21
/// 注意:
///
/// 假设我们的环境只能存储得下 32 位的有符号整数，则其数值范围为 [−231,  231 − 1]。请根据这个假设，如果反转后整数溢出那么就返回 0。
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/reverse-integer
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn reverse(mut x: i32) -> i32 {
    let mut res = 0_i64;
    while x != 0 {
        let pop = x % 10;
        if res > (std::i32::MAX / 10) as i64 || (res == (std::i32::MAX / 10) as i64 && pop > 7) { return 0; }
        if res < (std::i32::MIN / 10) as i64 || (res == (std::i32::MIN / 10) as i64 && pop < -8) { return 0; }
        res = res * 10 + pop as i64;
        x /= 10;
    }
    res as i32
}

/// 8.
///
/// 首先，该函数会根据需要丢弃无用的开头空格字符，直到寻找到第一个非空格的字符为止。
///
/// 当我们寻找到的第一个非空字符为正或者负号时，则将该符号与之后面尽可能多的连续数字组合起来，作为该整数的正负号；
///
/// 假如第一个非空字符是数字，则直接将其与之后连续的数字字符组合起来，形成整数。
///
/// 该字符串除了有效的整数部分之后也可能会存在多余的字符，这些字符可以被忽略，它们对于函数不应该造成影响。
///
/// 注意：假如该字符串中的第一个非空格字符不是一个有效整数字符、字符串为空或字符串仅包含空白字符时，则你的函数不需要进行转换。
///
/// 在任何情况下，若函数不能进行有效的转换时，请返回 0。
///
/// 说明：
///
/// 假设我们的环境只能存储 32 位大小的有符号整数，那么其数值范围为 [−231,  231 − 1]。如果数值超过这个范围，请返回  INT_MAX (231 − 1) 或 INT_MIN (−231) 。
///
/// 示例 1:
///
/// 输入: "42"
/// 输出: 42
/// 示例 2:
///
/// 输入: "   -42"
/// 输出: -42
/// 解释: 第一个非空白字符为 '-', 它是一个负号。
///      我们尽可能将负号与后面所有连续出现的数字组合起来，最后得到 -42 。
/// 示例 3:
///
/// 输入: "4193 with words"
/// 输出: 4193
/// 解释: 转换截止于数字 '3' ，因为它的下一个字符不为数字。
/// 示例 4:
///
/// 输入: "words and 987"
/// 输出: 0
/// 解释: 第一个非空字符是 'w', 但它不是数字或正、负号。
///      因此无法执行有效的转换。
/// 示例 5:
///
/// 输入: "-91283472332"
/// 输出: -2147483648
/// 解释: 数字 "-91283472332" 超过 32 位有符号整数范围。
///      因此返回 INT_MIN (−231) 。
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/string-to-integer-atoi
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn my_atoi(s: String) -> i32 {
    let (mut negative, mut res) = (false, 0_i64);
    for (i, ch) in s.trim().chars().enumerate() {
        if i == 0 && ch == '+' { continue; }
        if i == 0 && ch == '-' {
            negative = true;
            continue;
        }
        if !ch.is_digit(10) { break; } else {
            let pop = ch.to_digit(10).unwrap() as i64;
            if negative {
                if -res < (std::i32::MIN / 10) as i64 || (-res == (std::i32::MIN / 10) as i64 && -pop < -8) { return std::i32::MIN; }
            } else {
                if res > (std::i32::MAX / 10) as i64 || (res == (std::i32::MAX / 10) as i64 && pop > 7) { return std::i32::MAX; }
            }
            res = res * 10 + pop;
        }
    }
    (if negative { -res } else { res }) as i32
}

/// 9.
///
/// 判断一个整数是否是回文数。回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。
///
/// 示例 1:
///
/// 输入: 121
/// 输出: true
/// 示例 2:
///
/// 输入: -121
/// 输出: false
/// 解释: 从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。
/// 示例 3:
///
/// 输入: 10
/// 输出: false
/// 解释: 从右向左读, 为 01 。因此它不是一个回文数。
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/palindrome-number
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) { return false; }
    let (mut reverse, mut y) = (0, x);
    while y != 0 {
        reverse = reverse * 10 + y % 10;
        y /= 10;
    }
    x == reverse
}

/// 10.
///
/// 给你一个字符串 s 和一个字符规律 p，请你来实现一个支持 '.' 和 '*' 的正则表达式匹配。
///
/// '.' 匹配任意单个字符
/// '*' 匹配零个或多个前面的那一个元素
/// 所谓匹配，是要涵盖 整个 字符串 s的，而不是部分字符串。
///
/// 说明:
///
/// s 可能为空，且只包含从 a-z 的小写字母。
/// p 可能为空，且只包含从 a-z 的小写字母，以及字符 . 和 *。
/// 示例 1:
///
/// 输入:
/// s = "aa"
/// p = "a"
/// 输出: false
/// 解释: "a" 无法匹配 "aa" 整个字符串。
/// 示例 2:
///
/// 输入:
/// s = "aa"
/// p = "a*"
/// 输出: true
/// 解释: 因为 '*' 代表可以匹配零个或多个前面的那一个元素, 在这里前面的元素就是 'a'。因此，字符串 "aa" 可被视为 'a' 重复了一次。
/// 示例 3:
///
/// 输入:
/// s = "ab"
/// p = ".*"
/// 输出: true
/// 解释: ".*" 表示可匹配零个或多个（'*'）任意字符（'.'）。
/// 示例 4:
///
/// 输入:
/// s = "aab"
/// p = "c*a*b"
/// 输出: true
/// 解释: 因为 '*' 表示零个或多个，这里 'c' 为 0 个, 'a' 被重复一次。因此可以匹配字符串 "aab"。
/// 示例 5:
///
/// 输入:
/// s = "mississippi"
/// p = "mis*is*p*."
/// 输出: false
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/regular-expression-matching
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

///倒序动态规划
pub fn is_match(s: String, p: String) -> bool {
    let s_len = s.len();
    let p_len = p.len();
    let mut dp = vec![vec![false; p_len + 1]; s_len + 1];
    dp[0][0] = true;
    for i in 0..p_len {
        if p.get(i..=i).unwrap() == "*" && dp[0][i - 1] {
            dp[0][i + 1] = true;
        }
    }
    for i in 0..s_len {
        for j in 0..p_len {
            if p.get(j..=j).unwrap() == "." || p.get(j..=j).unwrap() == s.get(i..=i).unwrap() {
                dp[i + 1][j + 1] = dp[i][j];
            }
            if p.get(j..=j).unwrap() == "*" {
                if p.get(j - 1..=j - 1).unwrap() != s.get(i..=i).unwrap() && p.get(j - 1..=j - 1).unwrap() != "." {
                    dp[i + 1][j + 1] = dp[i + 1][j - 1];
                } else {
                    dp[i + 1][j + 1] = dp[i + 1][j] || dp[i][j + 1] || dp[i + 1][j - 1];
                }
            }
        }
    }
    dp[s_len][p_len]
}

#[cfg(test)]
mod test {
    use crate::{convert, reverse, my_atoi, is_palindrome, is_match};

    #[test]
    fn test_convert() {
        let res = convert("LEETCODEISHIRING".to_string(), 3);
        assert_eq!(res, String::from("LCIRETOESIIGEDHN"));
    }

    #[test]
    fn test_revers() {
        let res = reverse(123);
        assert_eq!(res, 321);
    }

    #[test]
    fn test_my_atoi() {
        let res = my_atoi(String::from("   -42"));
        assert_eq!(res, -42);
    }

    #[test]
    fn test_is_palindrome() {
        let res = is_palindrome(121);
        debug_assert!(res);
        let res = is_palindrome(123);
        debug_assert!(!res);
    }

    #[test]
    fn test_is_match() {
        assert!(is_match("a".to_string(), ".".to_string()));
        assert!(is_match("aaa".to_string(), "ab*ac*a".to_string()));
        assert!(is_match("aaaa".to_string(), ".*".to_string()));
        assert!(is_match("aa".to_string(), "a*".to_string()));
        assert!(is_match("ab".to_string(), ".*".to_string()));
        assert!(is_match("aab".to_string(), "c*a*b".to_string()));
        assert!(is_match("".to_string(), ".*".to_string()));
        assert!(is_match("".to_string(), "".to_string()));

        assert!(!is_match("".to_string(), "a".to_string()));
        assert!(!is_match("a".to_string(), "".to_string()));
        assert!(!is_match("aa".to_string(), "a".to_string()));
        assert!(!is_match("".to_string(), ".".to_string()));
        assert!(!is_match("mississippi".to_string(), "mis*is*p*.".to_string()));
    }
}
