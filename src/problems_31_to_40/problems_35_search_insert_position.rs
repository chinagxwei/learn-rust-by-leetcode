/// 35.
///
/// 给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。
///
/// 你可以假设数组中无重复元素。
///
/// 示例 1:
///
/// 输入: [1,3,5,6], 5
/// 输出: 2
/// 示例 2:
///
/// 输入: [1,3,5,6], 2
/// 输出: 1
/// 示例 3:
///
/// 输入: [1,3,5,6], 7
/// 输出: 4
/// 示例 4:
///
/// 输入: [1,3,5,6], 0
/// 输出: 0
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/search-insert-position
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

/// 暴力法
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    for (i, num) in nums.iter().enumerate() {
        if *num == target {
            return i as i32;
        } else if *num > target {
            return i as i32;
        }
    }
    nums.len() as i32
}

/// 二分法
pub fn search_insert_2(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, (nums.len() - 1) as i32);
    while left <= right {
        let middle = (left + right) / 2;
        if nums[middle as usize] == target {
            return middle;
        } else if nums[middle as usize] < target {
            left = middle + 1;
        } else {
            right = middle - 1;
        }
    }
    return left;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert() {
        let res = search_insert(vec![1, 3, 5, 6], 4);
        assert_eq!(res, 2);
        let res = search_insert_2(vec![1, 3, 5, 6], 2);
        assert_eq!(res, 1);
    }
}
