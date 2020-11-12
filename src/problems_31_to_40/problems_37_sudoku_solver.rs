/// 37.
/// 编写一个程序，通过填充空格来解决数独问题。
///
/// 一个数独的解法需遵循如下规则：
///
/// 数字 1-9 在每一行只能出现一次。
/// 数字 1-9 在每一列只能出现一次。
/// 数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。
/// 空白格用 '.' 表示。
///
/// 一个数独。
///
/// 答案被标成红色。
///
/// 提示：
///
/// 给定的数独序列只包含数字 1-9 和字符 '.' 。
/// 你可以假设给定的数独只有唯一解。
/// 给定数独永远是 9x9 形式的。
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/sudoku-solver
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
