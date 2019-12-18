/// 41.
/// 给定一个未排序的整数数组，找出其中没有出现的最小的正整数。
///
/// 示例 1:
///
/// 输入: [1,2,0]
/// 输出: 3
/// 示例 2:
///
/// 输入: [3,4,-1,1]
/// 输出: 2
/// 示例 3:
///
/// 输入: [7,8,9,11,12]
/// 输出: 1
/// 说明:
///
/// 你的算法的时间复杂度应为O(n)，并且只能使用常数级别的空间。
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/first-missing-positive
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///
pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let (nums_len, mut nums_mut) = (nums.len(), nums);
    for i in 0..nums_len {
        while nums_mut[i] != i as i32 + 1 {
            if nums_mut[i] <= 0 || nums_mut[i] > nums_len as i32 || nums_mut[i] == nums_mut[nums_mut[i] as usize - 1] {
                break;
            }

            let idx = nums_mut[i] - 1;
            nums_mut[i] = nums_mut[idx as usize];
            nums_mut[idx as usize] = idx + 1;
        }
    }
    for i in 0..nums_len {
        if nums_mut[i] != i as i32 + 1 { return i as i32 + 1; }
    }
    nums_len as i32 + 1
}

/// 42.
///
/// 给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。
///
///
///
/// 上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，可以接 6 个单位的雨水（蓝色部分表示雨水）。 感谢 Marcos 贡献此图。
///
/// 示例:
///
/// 输入: [0,1,0,2,1,0,1,3,2,1,2,1]
/// 输出: 6
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/trapping-rain-water
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///
pub fn trap(height: Vec<i32>) -> i32 {
    if height.len() == 0 { return 0; }
    let (mut left, mut right, mut ans, mut left_max, mut right_max) = (0, height.len() - 1, 0, 0, 0);
    while left < right {
        if height[left] < height[right] {
            if left_max <= height[left] {
                left_max = height[left]
            } else {
                ans += left_max - height[left];
            };
            left += 1;
        } else {
            if right_max <= height[right] {
                right_max = height[right];
            } else {
                ans += right_max - height[right];
            }
            right -= 1;
        }
    }
    ans
}

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
    fn test_first_missing_positive() {
        let res = first_missing_positive(vec![1, 2, 0]);
        assert_eq!(res, 3);
        let res = first_missing_positive(vec![3, 4, -1, 1]);
        assert_eq!(res, 2);
        let res = first_missing_positive(vec![7, 8, 9, 11, 12]);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_trap() {
        let res = trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]);
        assert_eq!(res, 6);
        let res = trap(vec![]);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_multiply() {
        let res = multiply("2".to_string(), "3".to_string());
        assert_eq!(res, "6");
        let res = multiply("123".to_string(), "456".to_string());
        assert_eq!(res, "56088");
    }
}

