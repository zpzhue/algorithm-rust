
#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut used = vec![vec![false; board[0].len()]; board.len()];
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Solution::backtrack(&board, (i, j), 0, &word.as_bytes(), &mut used) {
                    return true;
                }
            }
        }
        false
    }

    fn backtrack(
        board: &Vec<Vec<char>>, 
        index: (usize, usize),
        ci: usize,
        words: &[u8], 
        used: &mut Vec<Vec<bool>>
    ) -> bool {
        let (i, j) = index;
        if board[i][j] as u8 != words[ci] {
            return false;
        }

        if ci == words.len() - 1 {
            return true;
        }

        used[i][j] = true;
        let dir = [[-1, 0], [1, 0], [0, -1], [0, 1]];
        for [x, y] in dir{
            let i = x + i as i32;
            let j = y + j as i32;
            if i < 0 || j < 0 || i >= board.len() as i32 || j >= board[0].len() as i32 {
                continue;
            }
            let (i, j) = (i as usize, j as usize);
            if !used[i][j] && Solution::backtrack(board, (i, j), ci + 1, words, used) {
                return true;
            }
        }
        used[i][j] = false;
        false
    }


}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exist() {
        let board = vec![vec!['A','B','C','E'], vec!['S','F','C', 'S'], vec!['A','D','E','E']];
        let r = Solution::exist(board, String::from("ABCCED"));
        assert_eq!(r, true);
    }
}