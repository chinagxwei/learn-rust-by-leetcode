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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap() {
        let res = trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]);
        assert_eq!(res, 6);
        let res = trap(vec![]);
        assert_eq!(res, 0);
    }
}
