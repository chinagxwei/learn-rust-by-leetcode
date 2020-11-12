/// 1.
///
/// 给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
///
/// 你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素。
///
/// 示例:
///
/// 给定 nums = [2, 7, 11, 15], target = 9
///
/// 因为 nums[0] + nums[1] = 2 + 7 = 9
/// 所以返回 [0, 1]
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/two-sum
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for (i,v) in nums.into_iter().enumerate() {
            if hm.contains_key(&(target - v)) {
                return vec![*hm.get(&(target - v)).unwrap() as i32, i as i32]
            }else {
                hm.insert(v,i as i32);
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(nums, target);

        assert_eq!(result, vec![0, 1])
    }
}
