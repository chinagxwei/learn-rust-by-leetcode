/// 33.
///
/// 假设按照升序排序的数组在预先未知的某个点上进行了旋转。
///
/// ( 例如，数组 [0,1,2,4,5,6,7] 可能变为 [4,5,6,7,0,1,2] )。
///
/// 搜索一个给定的目标值，如果数组中存在这个目标值，则返回它的索引，否则返回 -1 。
///
/// 你可以假设数组中不存在重复的元素。
///
/// 你的算法时间复杂度必须是 O(log n) 级别。
///
/// 示例 1:
///
/// 输入: nums = [4,5,6,7,0,1,2], target = 0
/// 输出: 4
/// 示例 2:
///
/// 输入: nums = [4,5,6,7,0,1,2], target = 3
/// 输出: -1
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/search-in-rotated-sorted-array
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn search_1(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, (nums.len() - 1) as i32);
    while left <= right {
        let middle = (left + right) / 2;
        let mut num = nums[middle as usize];
        if (nums[middle as usize] < nums[0]) == (target < nums[0]) {
            num = nums[middle as usize];
        } else {
            num = if target < nums[0] { std::i32::MIN } else { std::i32::MAX }
        }
        if num < target {
            left = middle + 1;
        } else if num > target {
            right = middle - 1;
        } else {
            return middle;
        }
    }
    -1
}

pub fn search_2(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, (nums.len() - 1) as i32);
    while left <= right {
        let middle = (left + right) / 2;
        if nums[left as usize] <= nums[middle as usize] {
            if nums[left as usize] <= target && target < nums[middle as usize] {
                right = middle - 1;
            } else {
                left = middle + 1;
            }
        } else {
            if nums[middle as usize] < target && target <= nums[right as usize] {
                left = middle + 1;
            } else {
                right = middle - 1;
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let res = search_1(vec![4, 5, 6, 7, 0, 1, 2], 0);
        assert_eq!(res, 4);
        let res = search_2(vec![4, 5, 6, 7, 0, 1, 2], 3);
        assert_eq!(res, -1);
    }
}
