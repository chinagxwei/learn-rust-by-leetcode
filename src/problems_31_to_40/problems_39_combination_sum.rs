/// 39.
///
/// 给定一个无重复元素的数组 candidates 和一个目标数 target ，找出 candidates 中所有可以使数字和为 target 的组合。
///
/// candidates 中的数字可以无限制重复被选取。
///
/// 说明：
///
/// 所有数字（包括 target）都是正整数。
/// 解集不能包含重复的组合。 
/// 示例 1:
///
/// 输入: candidates = [2,3,6,7], target = 7,
/// 所求解集为:
/// [
///   [7],
///   [2,2,3]
/// ]
/// 示例 2:
///
/// 输入: candidates = [2,3,5], target = 8,
/// 所求解集为:
/// [
///   [2,2,2,2],
///   [2,3,3],
///   [3,5]
/// ]
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/combination-sum
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let (mut result_mut, mut candidates_mut, mut path_mut) = (vec![], candidates, vec![]);
    candidates_mut.sort_unstable();
    fn dfs(lists: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, mut path: Vec<i32>, start: i32, r_target: i32) {
        if r_target == 0 {
            res.push(path);
            return;
        }
        let mut index = start;
        while index < lists.len() as i32 && r_target - lists[index as usize] >= 0 {
            path.push(lists[index as usize]);
            dfs(lists, res, path.clone(), index, r_target - lists[index as usize]);
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
    fn test_combination_sum() {
        let res = combination_sum(vec![2, 3, 6, 7], 7);
        assert_eq!(res, vec![vec![2, 2, 3], vec![7]]);
    }
}
