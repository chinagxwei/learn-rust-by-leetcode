/// 34.
///
/// 给定一个按照升序排列的整数数组 nums，和一个目标值 target。找出给定目标值在数组中的开始位置和结束位置。
///
/// 你的算法时间复杂度必须是 O(log n) 级别。
///
/// 如果数组中不存在目标值，返回 [-1, -1]。
///
/// 示例 1:
///
/// 输入: nums = [5,7,7,8,8,10], target = 8
/// 输出: [3,4]
/// 示例 2:
///
/// 输入: nums = [5,7,7,8,8,10], target = 6
/// 输出: [-1,-1]
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/find-first-and-last-position-of-element-in-sorted-array
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///
pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() == 0 { return vec![-1, -1]; }
    let (mut left, mut right, mut result) = (0_i32, (nums.len() - 1) as i32, vec![]);
    while left < right {
        let middle = (left + right) / 2;
        if target <= nums[middle as usize] {
            right = middle;
        } else {
            left = middle + 1;
        }
    }
    result.push(if target == nums[left as usize] { left } else { -1 });
    right = (nums.len() - 1) as i32;
    while left < right {
        let middle = (left + right + 1) / 2;
        if target == nums[middle as usize] {
            left = middle;
        } else {
            right = middle - 1;
        }
    }
    result.push(if target == nums[right as usize] { right } else { -1 });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_range() {
        let res = search_range(vec![5, 7, 7, 8, 8, 10], 8);
        assert_eq!(res, [3, 4]);
        let res = search_range(vec![5, 7, 7, 8, 8, 10], 6);
        assert_eq!(res, [-1, -1]);
    }
}
