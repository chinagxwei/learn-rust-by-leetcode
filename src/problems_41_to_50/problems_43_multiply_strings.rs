/// 43.
///
/// 给定两个以字符串形式表示的非负整数 num1 和 num2，返回 num1 和 num2 的乘积，它们的乘积也表示为字符串形式。
///
/// 示例 1:
///
/// 输入: num1 = "2", num2 = "3"
/// 输出: "6"
/// 示例 2:
///
/// 输入: num1 = "123", num2 = "456"
/// 输出: "56088"
/// 说明：
///
/// num1 和 num2 的长度小于110。
/// num1 和 num2 只包含数字 0-9。
/// num1 和 num2 均不以零开头，除非是数字 0 本身。
/// 不能使用任何标准库的大数类型（比如 BigInteger）或直接将输入转换为整数来处理。
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/multiply-strings
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///
pub fn multiply(num1: String, num2: String) -> String {
    if num1 == String::from("0") || num2 == String::from("0") { return String::from("0"); }
    let (num1_bytes, num2_bytes) = (num1.as_bytes(), num2.as_bytes());
    let mut result = vec![b'0'; num1_bytes.len() + num2_bytes.len() + 1];
    let mut current = 0;
    for i in (0..num1_bytes.len()).rev() {
        let i_current = num1_bytes[i] - b'0';
        let mut result_current = result.len() - 1 - current;
        for j in (0..num2_bytes.len()).rev() {
            let j_current = num2_bytes[j] - b'0';
            let mul = j_current * i_current + result[result_current] - b'0';
            result[result_current] = mul % 10 + b'0';
            result[result_current - 1] = result[result_current - 1] - b'0' + mul / 10 + b'0';
            result_current -= 1;
        }
        current += 1;
    }
    for i in 0..result.len() {
        if result[i] != b'0' {
            current = i;
            break;
        }
    }
    String::from_utf8(result)
        .unwrap()
        .get(current..)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        let res = multiply("2".to_string(), "3".to_string());
        assert_eq!(res, "6");
        let res = multiply("123".to_string(), "456".to_string());
        assert_eq!(res, "56088");
    }
}
