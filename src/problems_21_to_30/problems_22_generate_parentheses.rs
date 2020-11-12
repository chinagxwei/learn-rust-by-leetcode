/// 22.
///
/// 给出 n 代表生成括号的对数，请你写出一个函数，使其能够生成所有可能的并且有效的括号组合。
///
/// 例如，给出 n = 3，生成结果为：
///
/// [
///   "((()))",
///   "(()())",
///   "(())()",
///   "()(())",
///   "()()()"
/// ]
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/generate-parentheses
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

/// 回溯法
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn backtrack(ans: &mut Vec<String>, current: &str, open: i32, close: i32, max: i32) {
        if current.len() == (max * 2) as usize {
            ans.push(current.to_string())
        } else {
            if open < max {
                let mut cur = current.to_string();
                cur.push_str("(");
                backtrack(ans, &cur, open + 1, close, max)
            }
            if close < open {
                let mut cur = current.to_string();
                cur.push_str(")");
                backtrack(ans, &cur, open, close + 1, max)
            }
        }
    }
    let mut ans = vec![];
    backtrack(&mut ans, "", 0, 0, n);
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_parenthesis() {
        let res = generate_parenthesis(2);
        println!("{:?}", res);
        assert_eq!(res, ["(())", "()()"])
    }
}
