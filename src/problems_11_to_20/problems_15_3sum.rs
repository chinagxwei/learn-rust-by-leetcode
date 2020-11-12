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
            let sum = nums[i] + nums[left] + nums[right];
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
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        let res = three_sum([-1, 0, 1, 2, -1, -4].to_vec());
        assert_eq!(res, [[-1, -1, 2],[-1, 0, 1]]);
        let res = three_sum([-2, 0, 0, 2, 2].to_vec());
        assert_eq!(res, [[-2, 0, 2]]);
    }
}
