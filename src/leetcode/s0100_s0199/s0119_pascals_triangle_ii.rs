/**
119. 杨辉三角 II
给定一个非负索引 rowIndex，返回「杨辉三角」的第 rowIndex 行。

在「杨辉三角」中，每个数是它左上方和右上方的数的和。


示例 1:
输入: rowIndex = 3
输出: [1,3,3,1]


示例 2:
输入: rowIndex = 0
输出: [1]


示例 3:
输入: rowIndex = 1
输出: [1,1]

提示:
0 <= rowIndex <= 33

进阶：
你可以优化你的算法到 O(rowIndex) 空间复杂度吗？

通过次数261,212提交次数379,022
*/



#[allow(unused)]
pub struct Solution;



#[allow(unused)]
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let n = (row_index + 1) as usize;
        let mut res = vec![1; n];
        for i in 2..n {
            for j in (1..i).rev() {
                res[j as usize] += res[j as usize - 1];
            }
        }
        res
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gen1() {
        let arr1 = Solution::get_row(3);
        assert_eq!(arr1, vec![1, 3, 3, 1]);
    }

    #[test]
    fn test2() {
        let arr2 = Solution::get_row(4);
        assert_eq!(arr2, vec![1, 4, 6, 4, 1]);
    }
}