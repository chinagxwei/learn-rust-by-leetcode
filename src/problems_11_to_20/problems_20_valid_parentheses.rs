/// 20.
///
/// 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串，判断字符串是否有效。
///
/// 有效字符串需满足：
///
/// 左括号必须用相同类型的右括号闭合。
/// 左括号必须以正确的顺序闭合。
/// 注意空字符串可被认为是有效字符串。
///
/// 示例 1:
///
/// 输入: "()"
/// 输出: true
/// 示例 2:
///
/// 输入: "()[]{}"
/// 输出: true
/// 示例 3:
///
/// 输入: "(]"
/// 输出: false
/// 示例 4:
///
/// 输入: "([)]"
/// 输出: false
/// 示例 5:
///
/// 输入: "{[]}"
/// 输出: true
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/valid-parentheses
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn is_valid(s: String) -> bool {
    if s.len() % 2 != 0 { return false; }
    let get_brackets_val = |num: char| -> Option<char> {
        match num {
            '(' => Some(')'),
            '[' => Some(']'),
            '{' => Some('}'),
            _ => Some('?')
        }
    };
    let (left, mut stack) = ("([{", vec![]);
    for ch in s.chars() {
        if left.find(ch).is_some() {
            stack.push(ch);
        } else {
            if get_brackets_val(stack.pop().unwrap_or('?')).unwrap() != ch {
                return false;
            }
        }
    }
    stack.len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let res = is_valid("()".to_string());
        assert!(res);
        let res = is_valid("()[]{}".to_string());
        assert!(res);
        let res = is_valid("{[]}".to_string());
        assert!(res);
        let res = is_valid("(]".to_string());
        assert!(!res);
        let res = is_valid("([)]".to_string());
        assert!(!res);
        let res = is_valid("]".to_string());
        assert!(!res);
        let res = is_valid("){".to_string());
        assert!(!res);
    }
}
