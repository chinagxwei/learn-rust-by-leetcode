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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_combinations() {
        let res = letter_combinations("23".to_string());
        assert_eq!(res, ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);
        let res = letter_combinations("".to_string());
        assert_eq!(res.len(), 0)
    }
}
