/// 36.
///
/// 判断一个 9x9 的数独是否有效。只需要根据以下规则，验证已经填入的数字是否有效即可。
///
/// 数字 1-9 在每一行只能出现一次。
/// 数字 1-9 在每一列只能出现一次。
/// 数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。
///
///
/// 上图是一个部分填充的有效的数独。
///
/// 数独部分空格内已填入了数字，空白格用 '.' 表示。
///
/// 示例 1:
///
/// 输入:
/// [
///   ["5","3",".",".","7",".",".",".","."],
///   ["6",".",".","1","9","5",".",".","."],
///   [".","9","8",".",".",".",".","6","."],
///   ["8",".",".",".","6",".",".",".","3"],
///   ["4",".",".","8",".","3",".",".","1"],
///   ["7",".",".",".","2",".",".",".","6"],
///   [".","6",".",".",".",".","2","8","."],
///   [".",".",".","4","1","9",".",".","5"],
///   [".",".",".",".","8",".",".","7","9"]
/// ]
/// 输出: true
/// 示例 2:
///
/// 输入:
/// [
///   ["8","3",".",".","7",".",".",".","."],
///   ["6",".",".","1","9","5",".",".","."],
///   [".","9","8",".",".",".",".","6","."],
///   ["8",".",".",".","6",".",".",".","3"],
///   ["4",".",".","8",".","3",".",".","1"],
///   ["7",".",".",".","2",".",".",".","6"],
///   [".","6",".",".",".",".","2","8","."],
///   [".",".",".","4","1","9",".",".","5"],
///   [".",".",".",".","8",".",".","7","9"]
/// ]
/// 输出: false
/// 解释: 除了第一行的第一个数字从 5 改为 8 以外，空格内其他数字均与 示例1 相同。
///      但由于位于左上角的 3x3 宫内有两个 8 存在, 因此这个数独是无效的。
/// 说明:
///
/// 一个有效的数独（部分已被填充）不一定是可解的。
/// 只需要根据以上规则，验证已经填入的数字是否有效即可。
/// 给定数独序列只包含数字 1-9 和字符 '.' 。
/// 给定数独永远是 9x9 形式的。
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/valid-sudoku
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let (mut row, mut column, mut matrix) = ([[0; 9]; 9], [[0; 9]; 9], [[0; 9]; 9]);
    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] != '.' {
                let num = board[i][j].to_digit(10).unwrap() - 1;
                row[i][num as usize] += 1;
                column[j][num as usize] += 1;
                matrix[3 * (i / 3) + (j / 3)][num as usize] += 1;
                if row[i][num as usize] == 2 || column[j][num as usize] == 2 || matrix[3 * (i / 3) + (j / 3)][num as usize] == 2 {
                    return false;
                }
            }
        }
    }
    true
}

/// 37.
///
/// 编写一个程序，通过已填充的空格来解决数独问题。
///
/// 一个数独的解法需遵循如下规则：
///
/// 数字 1-9 在每一行只能出现一次。
/// 数字 1-9 在每一列只能出现一次。
/// 数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。
/// 空白格用 '.' 表示。
/// Note:
///
/// 给定的数独序列只包含数字 1-9 和字符 '.' 。
/// 你可以假设给定的数独只有唯一解。
/// 给定数独永远是 9x9 形式的。
///

/// 回溯
pub fn solve_sudoku_1(board: &mut Vec<Vec<char>>) {
    let mut rows = vec![vec![false; 10]; 9];
    let mut columns = vec![vec![false; 10]; 9];
    let mut matrix = vec![vec![false; 10]; 9];
    type M<T> = Vec<Vec<T>>;
    fn recusive_solve_sudoku(t_board: &mut M<char>, t_rows: &mut M<bool>, t_columns: &mut M<bool>, t_matrix: &mut M<bool>, mut t_row: usize, mut t_column: usize) -> bool {
        if t_column == t_board[0].len() {
            t_column = 0;
            t_row += 1;
            if t_row == t_board.len() {
                return true;
            }
        }
        if t_board[t_row][t_column] == '.' {
            for i in 1..=9 {
                let can = !(t_rows[t_row][i] || t_columns[t_column][i] || t_matrix[3 * (t_row / 3) + (t_column / 3)][i]);
                if can {
                    t_rows[t_row][i] = true;
                    t_columns[t_column][i] = true;
                    t_matrix[3 * (t_row / 3) + (t_column / 3)][i] = true;
                    t_board[t_row][t_column] = i.to_string().parse::<char>().unwrap();
                    if recusive_solve_sudoku(t_board, t_rows, t_columns, t_matrix, t_row, t_column + 1) {
                        return true;
                    } else {
                        t_board[t_row][t_column] = '.';
                        t_rows[t_row][i] = false;
                        t_columns[t_column][i] = false;
                        t_matrix[3 * (t_row / 3) + (t_column / 3)][i] = false;
                    }
                }
            }
        } else {
            return recusive_solve_sudoku(t_board, t_rows, t_columns, t_matrix, t_row, t_column + 1);
        }

        false
    }
    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] != '.' {
                let num = board[i][j].to_digit(10).unwrap();
                rows[i][num as usize] = true;
                columns[j][num as usize] = true;
                matrix[3 * (i / 3) + (j / 3)][num as usize] = true;
            }
        }
    }
    recusive_solve_sudoku(board, &mut rows, &mut columns, &mut matrix, 0, 0);
}

/// dfs + 剪枝 + 位运算
pub fn solve_sudoku_2(board: &mut Vec<Vec<char>>) {
    let (mut row, mut column) = (vec![511; 9], vec![511; 9]);
    let (mut ones, mut map) = (vec![0; 512], vec![-1_i32; 512]);
    let mut cell = vec![vec![511; 3]; 3];
    type M = Vec<i32>;
    type N<T> = Vec<Vec<T>>;
    fn low_bit(x: i32) -> i32 { -x & x };
    fn get(t_row: &M, t_column: &M, t_cell: &N<i32>, x: i32, y: i32) -> i32 {
        t_row[x as usize] & t_column[y as usize] & t_cell[x as usize / 3][y as usize / 3]
    };
    fn change_state(t_row: &mut M, t_column: &mut M, t_cell: &mut N<i32>, x: i32, y: i32, t: i32) {
        t_row[x as usize] ^= 1 << t;
        t_column[y as usize] ^= 1 << t;
        t_cell[x as usize / 3][y as usize / 3] ^= 1 << t;
    };
    fn fill_state(t_board: &mut N<char>, t_row: &mut M, t_column: &mut M, t_cell: &mut N<i32>) -> i32 {
        let mut cnt = 0;
        for i in 0..9 {
            for j in 0..9 {
                if t_board[i][j] != '.' {
                    let t = t_board[i][j].to_digit(10).unwrap() - 1;
                    change_state(t_row, t_column, t_cell, i as i32, j as i32, t as i32);
                } else {
                    cnt += 1;
                }
            }
        }
        cnt
    }
    fn dfs(t_board: &mut N<char>, t_row: &mut M, t_column: &mut M, t_cell: &mut N<i32>, t_ones: &mut M, t_map: &mut M, t_cnt: i32) -> bool {
        if t_cnt == 0 { return true; }

        let (mut min, mut x, mut y) = (10, 0, 0);

        for i in 0..9 {
            for j in 0..9 {
                if t_board[i][j] == '.' {
                    let k = t_ones[get(t_row, t_column, t_cell, i as i32, j as i32) as usize];
                    if k < min {
                        min = k;
                        x = i;
                        y = j;
                    }
                }
            }
        }

        let mut z = get(t_row, t_column, t_cell, x as i32, y as i32);

        while z != 0 {
            let low_z = low_bit(z);
            let w = t_map[low_z as usize];

            change_state(t_row, t_column, t_cell, x as i32, y as i32, w);

            t_board[x][y] = (w + 1).to_string().parse::<char>().unwrap();

            if dfs(t_board, t_row, t_column, t_cell, t_ones, t_map, t_cnt - 1) { return true; }

            change_state(t_row, t_column, t_cell, x as i32, y as i32, w);

            t_board[x][y] = '.';

            z ^= low_z;
        }
        false
    }

    for i in 0..9 {
        map[1 << i] = i as i32;
    }

    for i in 0..512 {
        let mut n = 0;
        let mut j = i;
        while j != 0 {
            n += 1;
            j ^= low_bit(j);
        }
        ones[i as usize] = n;
    }
    let cnt = fill_state(board, &mut row, &mut column, &mut cell);
    dfs(board, &mut row, &mut column, &mut cell, &mut ones, &mut map, cnt);
}

/// 38.
///
/// 报数序列是一个整数序列，按照其中的整数的顺序进行报数，得到下一个数。其前五项如下：
///
/// 1.     1
/// 2.     11
/// 3.     21
/// 4.     1211
/// 5.     111221
/// 1 被读作  "one 1"  ("一个一") , 即 11。
/// 11 被读作 "two 1s" ("两个一"）, 即 21。
/// 21 被读作 "one 2",  "one 1" （"一个二" ,  "一个一") , 即 1211。
///
/// 给定一个正整数 n（1 ≤ n ≤ 30），输出报数序列的第 n 项。
///
/// 注意：整数顺序将表示为一个字符串。
///
///  
///
/// 示例 1:
///
/// 输入: 1
/// 输出: "1"
/// 示例 2:
///
/// 输入: 4
/// 输出: "1211"
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/count-and-say
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
///
pub fn count_and_say(n: i32) -> String {
    let (mut num_str, mut result) = (String::from(""), String::from("1"));
    for i in 1..n {
        num_str = result;
        result = String::from("");
        let mut current_num = 0;
        while current_num < num_str.len() {
            let (mut count, mut next_num) = (0, current_num);
            while next_num < num_str.len() && &num_str[current_num..=current_num] == &num_str[next_num..=next_num] {
                next_num += 1;
                count += 1;
            }
            result.push(count.to_string().parse().unwrap());
            result.push_str(&num_str[current_num..=current_num]);
            current_num = next_num;
        }
    }
    String::from(result)
}

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
mod test {
    use crate::{is_valid_sudoku, solve_sudoku_1, solve_sudoku_2, count_and_say, combination_sum, combination_sum2};

    #[test]
    fn test_is_valid_sudoku() {
        let board = vec![
            vec!['.', '.', '4', '.', '.', '.', '6', '3', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['5', '.', '.', '.', '.', '.', '.', '9', '.'],
            vec!['.', '.', '.', '5', '6', '.', '.', '.', '.'],
            vec!['4', '.', '3', '.', '.', '.', '.', '.', '1'],
            vec!['.', '.', '.', '7', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '5', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.']
        ];
        let res = is_valid_sudoku(board);
        assert!(!res);
    }

    #[test]
    fn test_solve_sudoku() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ];
        solve_sudoku_1(&mut board);

        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ];
        solve_sudoku_2(&mut board);
    }

    #[test]
    fn test_count_and_say() {
        let res = count_and_say(4);
        assert_eq!(res, String::from("1211"));
        let res = count_and_say(5);
        assert_eq!(res, String::from("111221"));
        let res = count_and_say(6);
        assert_eq!(res, String::from("312211"));
    }

    #[test]
    fn test_combination_sum() {
        let res = combination_sum(vec![2, 3, 6, 7], 7);
        assert_eq!(res, vec![vec![2, 2, 3], vec![7]]);
    }

    #[test]
    fn test_combination_sum2() {
        let res = combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);
        assert_eq!(res, vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]);
    }
}
