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

#[cfg(test)]
mod tests {
    use super::*;

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
}