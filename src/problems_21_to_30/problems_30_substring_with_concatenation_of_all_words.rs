use std::collections::HashMap;

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
    let (mut word_len, words_len, mut result) = (0, words.len(), vec![]);
    if s.len() == 0 || words_len == 0 || words[0].len() > s.len() {
        return result;
    } else {
        word_len = words[0].len();
    }
    let (mut needs, mut window) = (HashMap::<String, i32>::new(), HashMap::<String, i32>::new());
    for i in 0..words_len {
        needs.entry(words[i].to_string()).and_modify(|e| *e += 1).or_insert(1);
    }
    let (mut left, mut right) = (0, 0);
    for i in 0..word_len {
        left = i;
        right = i;
        let mut match_count = 0;
        while right <= (s.len() - word_len) {
            let s_sub = &s[right..right + word_len];
            right += word_len;
            if needs.contains_key(s_sub) {
                window.entry(s_sub.to_string()).and_modify(|e| *e += 1).or_insert(1);
                if needs.contains_key(s_sub) && window.get(s_sub).unwrap() == needs.get(s_sub).unwrap() {
                    match_count += 1;
                }
                while left < right && match_count == needs.len() {
                    if (right - left) / word_len == words_len {
                        result.push(left as i32);
                    }
                    let s_sub_2 = &s[left..left + word_len];
                    left += word_len;
                    window.entry(s_sub_2.to_string()).and_modify(|e| *e -= 1);
                    if needs.contains_key(s_sub_2) && window.get(s_sub_2).unwrap() < needs.get(s_sub_2).unwrap() {
                        match_count -= 1;
                    }
                }
            } else {
                match_count = 0;
                left = right;
                window.clear();
            }
        }
        window.clear();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_substring() {
        let res = find_substring(String::from("barfoothefoobarman"), vec![String::from("foo"), String::from("bar")]);
        assert_eq!(res, [0, 9]);
        let res = find_substring(String::from("wordgoodgoodgoodbestword"), vec!["word".to_string(), "good".to_string(), "best".to_string(), "word".to_string()]);
        assert_eq!(res, []);
    }
}
