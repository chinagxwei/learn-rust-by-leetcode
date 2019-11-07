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

#[cfg(test)]
mod test {
    use crate::{convert, reverse};

    #[test]
    fn test_convert() {
        let res = convert("LEETCODEISHIRING".to_string(), 3);
        assert_eq!(res, String::from("LCIRETOESIIGEDHN"))
    }

    #[test]
    fn test_revers() {
        let res = reverse(123);
        assert_eq!(res, 321)
    }
}
