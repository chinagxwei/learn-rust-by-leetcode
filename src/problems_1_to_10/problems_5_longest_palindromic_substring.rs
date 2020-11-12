/// 5.
///
/// 给定一个字符串 s，找到 s 中最长的回文子串。你可以假设 s 的最大长度为 1000。
///
/// 示例 1：
///
/// 输入: "babad"
/// 输出: "bab"
/// 注意: "aba" 也是一个有效答案。
/// 示例 2：
///
/// 输入: "cbbd"
/// 输出: "bb"
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/longest-palindromic-substring
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

struct Solution;

impl Solution {
    ///
/// 马拉车算法
///
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 1 || s.len() > 1000 { return "".to_string(); }
        let str_format: String = {
            let mut str_vec = {
                s.chars().map(|ch| { ch.to_string() }).collect::<Vec<String>>()
            };
            str_vec.insert(0, "^".to_string());
            str_vec.push("$".to_string());
            str_vec.join("#")
        };
        let sss_len = str_format.len();
        let mut p = vec![0; sss_len];
        //c表示中心下标，r表示右侧半径
        let (mut c, mut r) = (0 as i32, 0);
        for (index, _) in str_format.chars().enumerate() {
            if index == 0 || index == (sss_len - 1) { continue; };
            let mirror: usize = (2 * c - index as i32) as usize;

            if r > (index) {
                p[index] = (r - index).min(p[mirror]);
            } else {
                p[index] = 0;
            }

            //判断中心字符两侧一样字符的数量
            let (mut left, mut right) = (index + 1 + p[index], index - 1 - p[index]);
            while str_format.get(left..=left).unwrap() == str_format.get(right..=right).unwrap() {
                p[index] += 1;
                left = index + 1 + p[index];
                right = index - 1 - p[index];
            }

            //判断是否移动记录的下标中心和更新右侧半径
            if (index + p[index]) > r {
                c = index as i32;
                r = index + p[index];
            }
        }

        //获取最大长度，和对应下标
        let (mut max_len, mut center_index) = (0, 0);
        for (i, v) in p.iter().enumerate() {
            if *v > max_len {
                max_len = *v;
                center_index = i;
            }
        }
        //求源字符串开头下标
        let start = (center_index - max_len) / 2;
        s.get(start..start + max_len).unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        let res = Solution::longest_palindrome("dabcba".to_string());
        assert_eq!(res, String::from("abcba"))
    }
}
