/**
 给你一个大小为 m x n 的矩阵 board 表示甲板，其中，每个单元格可以是一艘战舰 'X' 或者是一个空位 '.' ，返回在甲板 board 上放置的 战舰 的数量。

战舰 只能水平或者垂直放置在 board 上。换句话说，战舰只能按 1 x k（1 行，k 列）或 k x 1（k 行，1 列）的形状建造，其中 k 可以是任意大小。两艘战舰之间至少有一个水平或垂直的空位分隔 （即没有相邻的战舰）。

 m == board.length
n == board[i].length
1 <= m, n <= 200
board[i][j] 是 '.' 或 'X'

思路：因为只能是 1 x n或者 n x 1的形式存在，并且不能连续
    遍历二维数组，如果当前字符为'X', 只要满足它的上个或左个不为'X'就符合条件
 */
pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
    let mut res = 0;
    for (i, vec) in board.iter().enumerate() {
        for (j, &c) in vec.iter().enumerate() {
            if (c == 'X' && (i == 0 || board[i - 1][j] == '.') && (j == 0 || board[i][j - 1] == '.')) {
                res += 1;
            }
        }
    }
    res
}

#[test]
fn test_count_battleships() {
    let arr = [["X",".",".","X"],[".",".",".","X"],[".",".",".","X"]];
    assert_eq!(count_battleships(arr), 2);
}