#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (n, m) = (matrix.len(), matrix[0].len());
        let (mut x, mut y) = (false, false);

        for i in 0..m {
            if matrix[0][i] == 0 {
                x = true;
                break;
            }
        }

        for i in 0..n {
            if matrix[i][0] == 0 {
                y = true;
                break;
            }
        }

        for i in 1..n {
            for j in 1..m {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for i in 1..n {
            for j in 1..m {
                if matrix[0][j] == 0 || matrix[i][0] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        if x {
            for i in 0..m {
                matrix[0][i] = 0;
            }
        }

        if y {
            for i in 0..n {
                matrix[i][0] = 0;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_zeroes1() {
        let mut m = vec![vec![0, 1, 2, 0], vec![3 ,4 ,5 ,2], vec![1, 3, 1, 5]];
        Solution::set_zeroes(&mut m);
        println!("function set_zeroes run result: {:?}", m);
    }

    #[test]
    fn test_set_zeroes2() {
        let mut m = vec![vec![1, 0]];
        Solution::set_zeroes(&mut m);
        println!("function set_zeroes run result: {:?}", m);
    }

    #[test]
    fn test_set_zeroes3() {
        let mut m = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut m);
        println!("function set_zeroes run result: {:?}", m);
    }

    #[test]
    fn test_set_zeroes4() {
        let mut m = vec![vec![1, 0, 3]];
        Solution::set_zeroes(&mut m);
        println!("function set_zeroes run result: {:?}", m);
    }
}
