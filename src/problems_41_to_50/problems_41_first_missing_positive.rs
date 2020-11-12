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
}
