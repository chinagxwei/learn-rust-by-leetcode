/// 28.
///
/// 实现 strStr() 函数。
///
/// 给定一个 haystack 字符串和一个 needle 字符串，在 haystack 字符串中找出 needle 字符串出现的第一个位置 (从0开始)。如果不存在，则返回  -1。
///
/// 示例 1:
///
/// 输入: haystack = "hello", needle = "ll"
/// 输出: 2
/// 示例 2:
///
/// 输入: haystack = "aaaaa", needle = "bba"
/// 输出: -1
/// 说明:
///
/// 当 needle 是空字符串时，我们应当返回什么值呢？这是一个在面试中很好的问题。
///
/// 对于本题而言，当 needle 是空字符串时我们应当返回 0 。这与C语言的 strstr() 以及 Java的 indexOf() 定义相符。
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/implement-strstr
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn str_str(haystack: String, needle: String) -> i32 {
    let (haystack_len, needle_len) = (haystack.len(), needle.len());
    if needle_len <= 0 { return 0; }
    let (haystack_u8, needle_u8) = (haystack.as_bytes(), needle.as_bytes());
    for h in 0..haystack_len {
        if haystack_len - h < needle_len { break; }
        if haystack_u8[h] == needle_u8[0] {
            let mut len = needle_len;
            for n in 0..needle_len {
                if haystack_u8[h + n] == needle_u8[n] {
                    len -= 1;
                }
            }
            if len == 0 { return h as i32; }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_str() {
        let res = str_str(String::from("hello"), String::from("ll"));
        assert_eq!(res, 2);
        let res = str_str(String::from("hello"), String::from(""));
        assert_eq!(res, 0);
        let res = str_str(String::from("hello"), String::from("hello world"));
        assert_eq!(res, -1);
        let res = str_str(String::from("aaaaaa"), String::from("bbc"));
        assert_eq!(res, -1);
    }
}
