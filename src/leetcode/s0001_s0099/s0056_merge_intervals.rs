/**
    https://leetcode.cn/problems/merge-intervals/

以数组 intervals 表示若干个区间的集合，其中单个区间为 intervals[i] = [starti, endi] 。请你合并所有重叠的区间，并返回 一个不重叠的区间数组，该数组需恰好覆盖输入中的所有区间 。

 
示例 1：
输入：intervals = [[1,3],[2,6],[8,10],[15,18]]
输出：[[1,6],[8,10],[15,18]]
解释：区间 [1,3] 和 [2,6] 重叠, 将它们合并为 [1,6].

示例 2：
输入：intervals = [[1,4],[4,5]]
输出：[[1,5]]
解释：区间 [1,4] 和 [4,5] 可被视为重叠区间。
 

提示：
1 <= intervals.length <= 104
intervals[i].length == 2
0 <= starti <= endi <= 104

*/

#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        let mut result: Vec<Vec<i32>> = Vec::new();
        intervals.sort_by(|a: &Vec<i32> ,b: &Vec<i32>| -> std::cmp::Ordering { a[0].cmp(&b[0]) });
        
        for item in intervals.into_iter() {
            if result.len() == 0 {
                result.push(item);
                continue;
            }
            let n = result.len() - 1;
            let last = result.last().unwrap();
            if last[1] < item[0] {
                result.push(item);
            }else {
                result[n] = vec![last[0], last[1].max(item[1])];
            }
        }
        result
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_merge() {
        let s = vec![vec![1,3], vec![2,6], vec![8,10], vec![15,18]];
        let r = Solution::merge(s);
        assert_eq!(r, vec![vec![1, 6], vec![8, 10], vec![15, 18]])
    }

}