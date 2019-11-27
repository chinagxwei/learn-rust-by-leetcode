/// 26.
///
/// 给定一个排序数组，你需要在原地删除重复出现的元素，使得每个元素只出现一次，返回移除后数组的新长度。
///
/// 不要使用额外的数组空间，你必须在原地修改输入数组并在使用 O(1) 额外空间的条件下完成。
///
/// 示例 1:
///
/// 给定数组 nums = [1,1,2],
///
/// 函数应该返回新的长度 2, 并且原数组 nums 的前两个元素被修改为 1, 2。
///
/// 你不需要考虑数组中超出新长度后面的元素。
/// 示例 2:
///
/// 给定 nums = [0,0,1,1,1,2,2,3,3,4],
///
/// 函数应该返回新的长度 5, 并且原数组 nums 的前五个元素被修改为 0, 1, 2, 3, 4。
///
/// 你不需要考虑数组中超出新长度后面的元素。
/// 说明:
///
/// 为什么返回数值是整数，但输出的答案是数组呢?
///
/// 请注意，输入数组是以“引用”方式传递的，这意味着在函数里修改输入数组对于调用者是可见的。
///
/// 你可以想象内部操作如下:
///
/// nums 是以“引用”方式传递的。也就是说，不对实参做任何拷贝
/// int len = removeDuplicates(nums);
///
/// 在函数里修改输入数组对于调用者是可见的。
/// 根据你的函数返回的长度, 它会打印出数组中该长度范围内的所有元素。
/// for (int i = 0; i < len; i++) {
///     print(nums[i]);
/// }
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 { return 0; }
    let mut len = 0;
    for index in 1..nums.len() {
        if nums[len] != nums[index] {
            nums[len + 1] = nums[index];
            len += 1;
        }
    }
    (len + 1) as i32
}

/// 27.
///
/// 给定一个数组 nums 和一个值 val，你需要原地移除所有数值等于 val 的元素，返回移除后数组的新长度。
///
/// 不要使用额外的数组空间，你必须在原地修改输入数组并在使用 O(1) 额外空间的条件下完成。
///
/// 元素的顺序可以改变。你不需要考虑数组中超出新长度后面的元素。
///
/// 示例 1:
///
/// 给定 nums = [3,2,2,3], val = 3,
///
/// 函数应该返回新的长度 2, 并且 nums 中的前两个元素均为 2。
///
/// 你不需要考虑数组中超出新长度后面的元素。
/// 示例 2:
///
/// 给定 nums = [0,1,2,2,3,0,4,2], val = 2,
///
/// 函数应该返回新的长度 5, 并且 nums 中的前五个元素为 0, 1, 3, 0, 4。
///
/// 注意这五个元素可为任意顺序。
///
/// 你不需要考虑数组中超出新长度后面的元素。
/// 说明:
///
/// 为什么返回数值是整数，但输出的答案是数组呢?
///
/// 请注意，输入数组是以“引用”方式传递的，这意味着在函数里修改输入数组对于调用者是可见的。
///
/// 你可以想象内部操作如下:
///
/// nums 是以“引用”方式传递的。也就是说，不对实参作任何拷贝
/// int len = removeElement(nums, val);
///
/// 在函数里修改输入数组对于调用者是可见的。
/// 根据你的函数返回的长度, 它会打印出数组中该长度范围内的所有元素。
/// for (int i = 0; i < len; i++) {
///     print(nums[i]);
/// }
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/remove-element
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.len() == 0 { return 0; }
    let mut len = 0;
    for index in 0..nums.len() {
        if nums[index] != val {
            nums[len] = nums[index];
            len += 1;
        }
    }
    len as i32
}

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

/// 29.
///
/// 给定两个整数，被除数 dividend 和除数 divisor。将两数相除，要求不使用乘法、除法和 mod 运算符。
///
/// 返回被除数 dividend 除以除数 divisor 得到的商。
///
/// 示例 1:
///
/// 输入: dividend = 10, divisor = 3
/// 输出: 3
/// 示例 2:
///
/// 输入: dividend = 7, divisor = -3
/// 输出: -2
/// 说明:
///
/// 被除数和除数均为 32 位有符号整数。
/// 除数不为 0。
/// 假设我们的环境只能存储 32 位有符号整数，其数值范围是 [−231,  231 − 1]。本题中，如果除法结果溢出，则返回 231 − 1。
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/divide-two-integers
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn divide(dividend: i32, divisor: i32) -> i32 {
    if dividend == std::i32::MIN && divisor == -1 { return std::i32::MAX; }
    let (mut dividend_mut, mut divisor_mut, mut flag_mut) = (dividend, divisor, 1);
    if dividend_mut > 0 {
        dividend_mut = -dividend_mut;
    } else {
        flag_mut = -flag_mut;
    }
    if divisor_mut > 0 {
        divisor_mut = -divisor_mut
    } else {
        flag_mut = -flag_mut;
    }

    let (mut res, mut tmp, mut k) = (0, 0, 0);
    while dividend_mut <= divisor_mut {
        tmp = divisor_mut;
        k = 1;
        while dividend_mut <= tmp + tmp && tmp + tmp < 0 {
            tmp += tmp;
            k += k;
        }
        res = res + k;
        dividend_mut -= tmp
    }
    if flag_mut > 0 { res } else { -res }
}

/// 30.
///
/// 给定一个字符串 s 和一些长度相同的单词 words。找出 s 中恰好可以由 words 中所有单词串联形成的子串的起始位置。
///
/// 注意子串要与 words 中的单词完全匹配，中间不能有其他字符，但不需要考虑 words 中单词串联的顺序。
///
///  
///
/// 示例 1：
///
/// 输入：
///   s = "barfoothefoobarman",
///   words = ["foo","bar"]
/// 输出：[0,9]
/// 解释：
/// 从索引 0 和 9 开始的子串分别是 "barfoo" 和 "foobar" 。
/// 输出的顺序不重要, [9,0] 也是有效答案。
/// 示例 2：
///
/// 输入：
///   s = "wordgoodgoodgoodbestword",
///   words = ["word","good","best","word"]
/// 输出：[]
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/substring-with-concatenation-of-all-words
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    vec![0]
}

#[cfg(test)]
mod test {
    use crate::{remove_duplicates, remove_element, str_str, divide, find_substring};

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 2];
        let res = remove_duplicates(&mut nums);
        assert_eq!(res, 2);
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let res = remove_duplicates(&mut nums);
        assert_eq!(res, 5)
    }

    #[test]
    fn test_remove_element() {
        let mut nums = vec![3, 2, 2, 3];
        let res = remove_element(&mut nums, 3);
        assert_eq!(res, 2);
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let res = remove_element(&mut nums, 2);
        assert_eq!(res, 5)
    }

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

    #[test]
    fn test_divide() {
        let res = divide(10, 3);
        assert_eq!(res, 3);
        let res = divide(7, -3);
        assert_eq!(res, -2);
    }

    #[test]
    fn test_find_substring() {
        let res = find_substring(String::from("barfoothefoobarman"), vec![String::from("foo"), String::from("bar")]);
    }
}
