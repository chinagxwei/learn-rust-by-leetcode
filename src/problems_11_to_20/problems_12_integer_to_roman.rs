/// 12.
///
/// 罗马数字包含以下七种字符： I， V， X， L，C，D 和 M。
///
/// 字符          数值
/// I             1
/// V             5
/// X             10
/// L             50
/// C             100
/// D             500
/// M             1000
/// 例如， 罗马数字 2 写做 II ，即为两个并列的 1。12 写做 XII ，即为 X + II 。 27 写做  XXVII, 即为 XX + V + II 。
///
/// 通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 IIII，而是 IV。数字 1 在数字 5 的左边，
/// 所表示的数等于大数 5 减小数 1 得到的数值 4 。同样地，数字 9 表示为 IX。这个特殊的规则只适用于以下六种情况：
///
/// I 可以放在 V (5) 和 X (10) 的左边，来表示 4 和 9。
/// X 可以放在 L (50) 和 C (100) 的左边，来表示 40 和 90。 
/// C 可以放在 D (500) 和 M (1000) 的左边，来表示 400 和 900。
/// 给定一个整数，将其转为罗马数字。输入确保在 1 到 3999 的范围内。
///
/// 示例 1:
///
/// 输入: 3
/// 输出: "III"
/// 示例 2:
///
/// 输入: 4
/// 输出: "IV"
/// 示例 3:
///
/// 输入: 9
/// 输出: "IX"
/// 示例 4:
///
/// 输入: 58
/// 输出: "LVIII"
/// 解释: L = 50, V = 5, III = 3.
/// 示例 5:
///
/// 输入: 1994
/// 输出: "MCMXCIV"
/// 解释: M = 1000, CM = 900, XC = 90, IV = 4.
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/integer-to-roman
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

/// 贪心算法
pub fn int_to_roman_1(mut num: i32) -> String {
    let nums = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let romans = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
    let mut roman_str = String::new();
    for i in 0..nums.len() {
        while num >= nums[i] {
            roman_str.push_str(romans[i]);
            num -= nums[i];
        }
    }
    roman_str
}

/// 暴力算法
pub fn int_to_roman_2(num: i32) -> String {
    let q = ["", "M", "MM", "MMM"];
    let b = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
    let s = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
    let g = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
    let mut roman_str = String::new();
    roman_str.push_str(q[(num / 1000) as usize]);
    roman_str.push_str(b[(num % 1000 / 100) as usize]);
    roman_str.push_str(s[(num % 100 / 10) as usize]);
    roman_str.push_str(g[(num % 10) as usize]);
    roman_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_roman() {
        let res = int_to_roman_1(1994);
        assert_eq!(res, String::from("MCMXCIV"));
        let res = int_to_roman_2(58);
        assert_eq!(res, String::from("LVIII"));
    }
}
