
#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut res = 0;
        let m = grid.len();
        let n = grid[0].len();
        let mut grid = grid;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    // 如果碰到岛屿就 +1
                    res += 1;
                    // 然后把岛屿淹没
                    Solution::dfs(&mut grid, m, n, i, j);
                }
            }
        }

        res
    }
    
    fn dfs(grid: &mut Vec<Vec<char>>, m: usize, n: usize, i: usize, j: usize) {
        if i < m && j < n && grid[i][j] == '1' {
            grid[i][j] = '0';

            if i > 0 {
                Solution::dfs(grid, m, n, i - 1, j);
            }
            Solution::dfs(grid, m, n, i + 1, j);
            if j > 0 {
                Solution::dfs(grid, m, n, i, j - 1); 
            }
            Solution::dfs(grid, m, n, i, j + 1);
        } 
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_islands(){
        let lands = vec![
            vec!['1', '1', '1', '1', '0'], 
            vec!['1', '1', '0', '1', '0'], 
            vec!['1', '1', '0', '0', '0'], 
            vec!['0', '0', '0', '0', '0']
        ];
        let r = Solution::num_islands(lands);
        assert_eq!(r, 1)
    }

    #[test]
    fn test_num_islands2(){
        let lands = vec![
            vec!['1', '1', '0', '0', '0'], 
            vec!['1', '1', '0', '0', '0'], 
            vec!['0', '0', '1', '0', '0'], 
            vec!['0', '0', '0', '1', '1']
        ];
        let r = Solution::num_islands(lands);
        assert_eq!(r, 3)
    }

    #[test]
    fn test_num_islands3(){
        let lands = vec![
            vec!['1', '1', '1'], 
            vec!['0', '1', '0'], 
            vec!['1', '1', '1']
        ];
        let r = Solution::num_islands(lands);
        assert_eq!(r, 1)
    }
}



