/// 18.
///
/// 给定一个包含 n 个整数的数组 nums 和一个目标值 target，判断 nums 中是否存在四个元素 a，b，c 和 d ，
/// 使得 a + b + c + d 的值与 target 相等？找出所有满足条件且不重复的四元组。
///
/// 注意：
///
/// 答案中不可以包含重复的四元组。
///
/// 示例：
///
/// 给定数组 nums = [1, 0, -1, 0, -2, 2]，和 target = 0。
///
/// 满足要求的四元组集合为：
/// [
///   [-1,  0, 0, 1],
///   [-2, -1, 1, 2],
///   [-2,  0, 0, 2]
/// ]
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/4sum
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///

pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let nums_len = nums.len();
    if nums_len < 4 { return vec![]; }
    nums.sort_unstable();
    ///
    /// 递归内部函数
    ///
    fn x_sum(x_nums: &Vec<i32>, stack: &mut Vec<i32>, stack_index: i32, k: usize, begin: i32, x_target: i32) -> Vec<Vec<i32>> {
        let mut x_ans = vec![];
        if k == 2 {
            let (mut left, mut right) = (begin, x_nums.len() - 1);
            while left < right as i32 {
                let mut tmp_ans = vec![];
                if x_nums[left as usize] + x_nums[right as usize] > x_target {
                    right -= 1;
                } else if x_nums[left as usize] + x_nums[right as usize] < x_target {
                    left += 1;
                } else {
                    stack[(stack_index + 1) as usize] = x_nums[left as usize];
                    stack[(stack_index + 2) as usize] = x_nums[right as usize];
                    for i in 0..=(stack_index + 2) {
                        tmp_ans.push(stack[i as usize]);
                    }
                    x_ans.push(tmp_ans);
                    right -= 1;
                    left += 1;
                    while left < right as i32 && x_nums[left as usize] == x_nums[left as usize - 1] {
                        left += 1;
                    }
                    while right as i32 > left && right < x_nums.len() - 1 && x_nums[right as usize] == x_nums[right as usize + 1] {
                        right -= 1;
                    }
                }
            }
        } else {
            for i in begin..(x_nums.len() - k + 1) as i32 {
                if i > begin && x_nums[i as usize] == x_nums[i as usize - 1] {
                    continue;
                }
                stack[(stack_index + 1) as usize] = x_nums[i as usize];
                let tmpe_ans = x_sum(x_nums, stack, stack_index + 1, k - 1, i + 1, x_target - x_nums[i as usize]);
                if tmpe_ans.len() <= 0 { continue; }
                x_ans.extend_from_slice(&tmpe_ans);
            }
        }
        x_ans
    }
    let k = 4;
    let mut current_stack = vec![0_i32; k];
    let stack_index = -1;
    let begin = 0;
    x_sum(&nums, &mut current_stack, stack_index, k, begin, target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_sum() {
        let res = four_sum([1, 0, -1, 0, -2, 2].to_vec(), 0);
        assert_eq!(res, [[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]])
    }
}
