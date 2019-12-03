/// 31.
///
/// 实现获取下一个排列的函数，算法需要将给定数字序列重新排列成字典序中下一个更大的排列。
///
/// 如果不存在下一个更大的排列，则将数字重新排列成最小的排列（即升序排列）。
///
/// 必须原地修改，只允许使用额外常数空间。
///
/// 以下是一些例子，输入位于左侧列，其相应输出位于右侧列。
/// 1,2,3 → 1,3,2
/// 3,2,1 → 1,2,3
/// 1,1,5 → 1,5,1
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/next-permutation
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///
pub fn next_permutation(nums: &mut Vec<i32>) {
    let (mut i, nums_len) = ((nums.len() - 2) as i32, nums.len());
    let swap = |tmp_nums: &mut Vec<i32>, i: usize, j: usize| {
        let tmp = tmp_nums[i];
        tmp_nums[i] = tmp_nums[j];
        tmp_nums[j] = tmp;
    };
    while i >= 0 && nums[(i + 1) as usize] <= nums[i as usize] {
        i -= 1;
    }
    if i >= 0 {
        let mut j = (nums_len - 1) as i32;
        while j >= 0 && nums[j as usize] <= nums[i as usize] {
            j -= 1
        }
        swap(nums, i as usize, j as usize);
    }
    let (mut start, mut end) = ((i + 1) as usize, nums_len - 1);
    while start < end {
        swap(nums, start, end);
        start += 1;
        end -= 1;
    }
}

#[cfg(test)]
mod test {
    use crate::next_permutation;

    #[test]
    fn test_next_permutation() {
        let mut input = vec![1, 2, 3];
        next_permutation(&mut input);
        assert_eq!(input, [1, 3, 2]);
        let mut input = vec![3, 2, 1];
        next_permutation(&mut input);
        assert_eq!(input, [1, 2, 3]);
        let mut input = vec![1, 1, 5];
        next_permutation(&mut input);
        assert_eq!(input, [1, 5, 1]);
    }
}
