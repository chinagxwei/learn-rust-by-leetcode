/// 14.
///
/// 编写一个函数来查找字符串数组中的最长公共前缀。
///
/// 如果不存在公共前缀，返回空字符串 ""。
///
/// 示例 1:
///
/// 输入: ["flower","flow","flight"]
/// 输出: "fl"
/// 示例 2:
///
/// 输入: ["dog","racecar","car"]
/// 输出: ""
/// 解释: 输入不存在公共前缀。
/// 说明:
///
/// 所有输入只包含小写字母 a-z 。
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/longest-common-prefix
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 0 { return String::from(""); }
    let mut ans = *&strs[0].as_str();
    for s in strs.iter() {
        let mut i = 0;
        while i < ans.len() && i < s.len() {
            if ans.get(i..=i).unwrap() != s.get(i..=i).unwrap() {
                break;
            }
            i += 1;
        }
        ans = ans.get(0..i).unwrap();
        if ans == "" {
            return String::from(ans);
        }
    }
    String::from(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        let res = longest_common_prefix(["flower".to_string(), "flow".to_string(), "flight".to_string()].to_vec());
        assert_eq!(res, "fl".to_string());
        let res = longest_common_prefix(["dog".to_string(), "racecar".to_string(), "car".to_string()].to_vec());
        assert_eq!(res, "".to_string());
    }
}
