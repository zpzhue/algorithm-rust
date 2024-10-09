
/**
    https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/

示例 1：
输入：nums = [5,7,7,8,8,10], target = 8
输出：[3,4]

示例 2：
输入：nums = [5,7,7,8,8,10], target = 6
输出：[-1,-1]

示例 3：
输入：nums = [], target = 0
输出：[-1,-1]

*/


#[allow(unused)]
pub struct Solution;


#[allow(unused)]
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();
        let search = |t| -> usize {
            let (mut l, mut r) = (0_usize, n);
            while l < r {
                let mid = (l + r) >> 1;
                if nums[mid] < t {
                    l = mid + 1
                }else {
                    r = mid
                }
            }
            l
        };
        let start = search(target) as i32;
        let end = search(target + 1) as i32;
        if start == end {
            return vec![-1, -1];
        }
        vec![start, end - 1]
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search_range() {
        let l = vec![5,7,7,8,8,10];
        let r = Solution::search_range(l, 8);
        assert_eq!(r, vec![3,4]);
    }

    #[test]
    fn test_search_range2() {
        let l = vec![5,7,7,8,8,10];
        let r = Solution::search_range(l, 6);
        assert_eq!(r, vec![-1,-1]);
    }

    #[test]
    fn test_search_range3() {
        let l = vec![];
        let r = Solution::search_range(l, 0);
        assert_eq!(r, vec![-1,-1]);
    }
}