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
    if num_rows == 1 { return s }
    let mut rows: Vec<String> = vec![];
    let take = num_rows.min(s.len() as i32);
    for _ in (0..take).rev() {
        rows.push(String::new())
    }
    let (mut cur_row, mut going_down) = (0_i32, false);
    for ch in s.chars() {
        rows[cur_row as usize].push(ch);
        if cur_row == 0 || cur_row == (num_rows - 1) { going_down = !going_down }
        cur_row += if going_down { 1 } else { -1 }
    }
    let mut ret = String::new();
    for row in rows {
        ret.push_str(row.as_str())
    }
    ret
}

#[cfg(test)]
mod test {
    use crate::convert;

    #[test]
    fn test_convert() {
        let res = convert("LEETCODEISHIRING".to_string(), 3);
        assert_eq!(res, String::from("LCIRETOESIIGEDHN"))
    }
}
