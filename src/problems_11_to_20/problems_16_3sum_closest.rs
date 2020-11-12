/// 16.
///
/// 给定一个包括 n 个整数的数组 nums 和 一个目标值 target。找出 nums 中的三个整数，使得它们的和与 target 最接近。返回这三个数的和。假定每组输入只存在唯一答案。
///
/// 例如，给定数组 nums = [-1，2，1，-4], 和 target = 1.
///
/// 与 target 最接近的三个数的和为 2. (-1 + 2 + 1 = 2).
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/3sum-closest
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    let nums_len = nums.len();
    if nums_len < 3 { return 0; }
    nums.sort_unstable();
    let mut ans = nums[0] + nums[1] + nums[2];
    for i in 0..nums_len {
        let (mut left, mut right) = (i + 1, nums_len - 1);
        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            if (target - sum).abs() < (target - ans).abs() {
                ans = sum;
            }
            if sum > target {
                right -= 1;
            } else if sum < target {
                left += 1;
            } else {
                return ans;
            }
        }
    }
    ans
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum_closest() {
        let res = three_sum_closest([-1, 2, 1, -4].to_vec(), 1);
        assert_eq!(res, 2)
    }
}
