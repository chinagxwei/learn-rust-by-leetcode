/// 40.
///
/// 给定一个数组 candidates 和一个目标数 target ，找出 candidates 中所有可以使数字和为 target 的组合。
///
/// candidates 中的每个数字在每个组合中只能使用一次。
///
/// 说明：
///
/// 所有数字（包括目标数）都是正整数。
/// 解集不能包含重复的组合。 
/// 示例 1:
///
/// 输入: candidates = [10,1,2,7,6,1,5], target = 8,
/// 所求解集为:
/// [
///   [1, 7],
///   [1, 2, 5],
///   [2, 6],
///   [1, 1, 6]
/// ]
/// 示例 2:
///
/// 输入: candidates = [2,5,2,1,2], target = 5,
/// 所求解集为:
/// [
///   [1,2,2],
///   [5]
/// ]
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/combination-sum-ii
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///
pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let (mut result_mut, mut candidates_mut, mut path_mut) = (vec![], candidates, vec![]);
    candidates_mut.sort_unstable();
    fn dfs(lists: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, mut path: Vec<i32>, start: i32, r_target: i32) {
        if r_target == 0 {
            res.push(path);
            return;
        }
        let mut index = start;
        while (index < lists.len() as i32) && (r_target - lists[index as usize] >= 0) {
            if r_target - lists[index as usize] < 0 { break; }
            if index > start && lists[index as usize] == lists[(index - 1) as usize] {
                index += 1;
                continue;
            }
            path.push(lists[index as usize]);
            dfs(lists, res, path.clone(), index + 1, r_target - lists[index as usize]);
            path.pop();
            index += 1;
        }
    }
    dfs(&mut candidates_mut, &mut result_mut, path_mut, 0, target);
    result_mut
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum2() {
        let res = combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);
        assert_eq!(res, vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]);
    }
}
