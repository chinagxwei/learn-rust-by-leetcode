/// 11.
///
/// 给定 n 个非负整数 a1，a2，...，an，每个数代表坐标中的一个点 (i, ai) 。在坐标内画 n 条垂直线，
///
/// 垂直线 i 的两个端点分别为 (i, ai) 和 (i, 0)。找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。
///
/// 说明：你不能倾斜容器，且 n 的值至少为 2。
///
///
///
/// 图中垂直线代表输入数组 [1,8,6,2,5,4,8,3,7]。在此情况下，容器能够容纳水（表示为蓝色部分）的最大值为 49。
///
///  
///
/// 示例:
///
/// 输入: [1,8,6,2,5,4,8,3,7]
/// 输出: 49
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/container-with-most-water
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn max_area(height: Vec<i32>) -> i32 {
    let h_len = height.len();
    if h_len < 2 { return -1; }
    let (mut left, mut right, mut area) = (0, h_len - 1, 0);
    while left < right {
        area = area.max(height[left].min(height[right]) * (right - left) as i32);
        if height[left] < height[right] {
            left += 1
        } else {
            right -= 1
        }
    }
    area
}

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

/// 13.
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
/// 给定一个罗马数字，将其转换成整数。输入确保在 1 到 3999 的范围内。
///
/// 示例 1:
///
/// 输入: "III"
/// 输出: 3
/// 示例 2:
///
/// 输入: "IV"
/// 输出: 4
/// 示例 3:
///
/// 输入: "IX"
/// 输出: 9
/// 示例 4:
///
/// 输入: "LVIII"
/// 输出: 58
/// 解释: L = 50, V= 5, III = 3.
/// 示例 5:
///
/// 输入: "MCMXCIV"
/// 输出: 1994
/// 解释: M = 1000, CM = 900, XC = 90, IV = 4.
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/roman-to-integer
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

pub fn roman_to_int(s: String) -> i32 {
    let get_roman_val = |roman_str: &str| -> Option<i32>{
        match roman_str {
            "I" => Some(1),
            "IV" => Some(4),
            "V" => Some(5),
            "IX" => Some(9),
            "X" => Some(10),
            "XL" => Some(40),
            "L" => Some(50),
            "XC" => Some(90),
            "C" => Some(100),
            "CD" => Some(400),
            "D" => Some(500),
            "CM" => Some(900),
            "M" => Some(1000),
            _ => None
        }
    };
    let (mut start, mut num, s_len) = (0, 0, s.len());
    while start < s_len {
        let key = s.get(start..=start + 1).unwrap_or_else(|| { s.get(start..=start).unwrap() });
        if let Some(v) = get_roman_val(key) {
            num += v;
            start += 2
        } else {
            let key = s.get(start..=start).unwrap();
            num += get_roman_val(key).unwrap_or(0);
            start += 1
        }
    }
    num
}

/// 14.
///
/// 编写一个函数来查找字符串数组中的最长公共前缀。
///
/// 如果不存在公共前缀，返回空字符串 ""。
///
/// 示例 1:
///
/// 输入: ["flower","flow","flight"]
/// 输出: "fl"
/// 示例 2:
///
/// 输入: ["dog","racecar","car"]
/// 输出: ""
/// 解释: 输入不存在公共前缀。
/// 说明:
///
/// 所有输入只包含小写字母 a-z 。
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/longest-common-prefix
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 0 { return String::from(""); }
    let mut ans = *&strs[0].as_str();
    for s in strs.iter() {
        let mut i = 0;
        while i < ans.len() && i < s.len() {
            if ans.get(i..=i).unwrap() != s.get(i..=i).unwrap() {
                break;
            }
            i += 1;
        }
        ans = ans.get(0..i).unwrap();
        if ans == "" {
            return String::from(ans);
        }
    }
    String::from(ans)
}

/// 15.
/// 给定一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，使得 a + b + c = 0 ？找出所有满足条件且不重复的三元组。
///
/// 注意：答案中不可以包含重复的三元组。
///
/// 例如, 给定数组 nums = [-1, 0, 1, 2, -1, -4]，
///
/// 满足要求的三元组集合为：
/// [
///   [-1, 0, 1],
///   [-1, -1, 2]
/// ]
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/3sum
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let (mut ans, nums_len) = (vec![], nums.len());
    if nums_len < 3 { return ans; }
    nums.sort_unstable();
    for i in 0..nums_len {
        if nums[i] > 0 { break; }
        if i > 0 && nums[i] == nums[i - 1] { continue; }
        let (mut left, mut right) = (i + 1, nums_len - 1);
        while left < right {
            let mut sum = nums[i] + nums[left] + nums[right];
            if sum == 0 {
                ans.push(vec![nums[i], nums[left], nums[right]]);
                while left < right && nums[left] == nums[left + 1] { left += 1; }
                while left < right && nums[right] == nums[right - 1] { right -= 1; }
                left += 1;
                right -= 1;
            } else if sum < 0 { left += 1 } else if sum > 0 { right -= 1 }
        }
    }
    ans
}


#[cfg(test)]
mod test {
    use crate::{max_area, int_to_roman_1, int_to_roman_2, roman_to_int, longest_common_prefix, three_sum};

    #[test]
    fn test_max_area() {
        let res = max_area([1, 8, 6, 2, 5, 4, 8, 3, 7].to_vec());
        assert_eq!(res, 49);
    }

    #[test]
    fn test_int_to_roman() {
        let res = int_to_roman_1(1994);
        assert_eq!(res, String::from("MCMXCIV"));
        let res = int_to_roman_2(58);
        assert_eq!(res, String::from("LVIII"));
    }

    #[test]
    fn test_roman_to_int() {
        let res = roman_to_int("IV".to_string());
        assert_eq!(res, 4);
        let res = roman_to_int("LVIII".to_string());
        assert_eq!(res, 58);
        let res = roman_to_int("MCMXCIV".to_string());
        assert_eq!(res, 1994);
    }

    #[test]
    fn test_longest_common_prefix() {
        let res = longest_common_prefix(["flower".to_string(), "flow".to_string(), "flight".to_string()].to_vec());
        assert_eq!(res, "fl".to_string());
        let res = longest_common_prefix(["dog".to_string(), "racecar".to_string(), "car".to_string()].to_vec());
        assert_eq!(res, "".to_string());
    }

    #[test]
    fn test_three_sum() {
        let res = three_sum([-1, 0, 1, 2, -1, -4].to_vec());
        assert_eq!(res, [[-1, -1, 2],[-1, 0, 1]]);
        let res = three_sum([-2, 0, 0, 2, 2].to_vec());
        assert_eq!(res, [[-2, 0, 2]]);
    }
}
