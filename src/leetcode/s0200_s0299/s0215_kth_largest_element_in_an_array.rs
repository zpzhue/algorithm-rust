
/**
https://leetcode.cn/problems/kth-largest-element-in-an-array/

215. 数组中的第K个最大元素
给定整数数组 nums 和整数 k，请返回数组中第 k 个最大的元素。

请注意，你需要找的是数组排序后的第 k 个最大的元素，而不是第 k 个不同的元素。

你必须设计并实现时间复杂度为 O(n) 的算法解决此问题。

 
示例 1:
输入: [3,2,1,5,6,4], k = 2
输出: 5

示例 2:
输入: [3,2,3,1,2,4,5,5,6], k = 4
输出: 4
 

提示：
1 <= k <= nums.length <= 105
-104 <= nums[i] <= 104
*/



#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        nums[nums.len() - k as usize]
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_kth_largest1(){
        let r = Solution::find_kth_largest(vec![3,2,1,5,6,4], 2);
        assert_eq!(r, 5);
    }

    #[test]
    fn test_find_kth_largest2(){
        let r = Solution::find_kth_largest(vec![3,2,3,1,2,4,5,5,6], 4);
        assert_eq!(r, 4);
    }
}