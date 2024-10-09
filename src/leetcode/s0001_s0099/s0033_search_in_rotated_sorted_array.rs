/**
    https://leetcode.cn/problems/search-in-rotated-sorted-array/


整数数组 nums 按升序排列，数组中的值 互不相同 。

在传递给函数之前，nums 在预先未知的某个下标 k（0 <= k < nums.length）上进行了 旋转，使数组变为 [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]]（下标 从 0 开始 计数）。例如， [0,1,2,4,5,6,7] 在下标 3 处经旋转后可能变为 [4,5,6,7,0,1,2] 。

给你 旋转后 的数组 nums 和一个整数 target ，如果 nums 中存在这个目标值 target ，则返回它的下标，否则返回 -1 。

你必须设计一个时间复杂度为 O(log n) 的算法解决此问题。

 

示例 1：
输入：nums = [4,5,6,7,0,1,2], target = 0
输出：4

示例 2：
输入：nums = [4,5,6,7,0,1,2], target = 3
输出：-1

示例 3：
输入：nums = [1], target = 0
输出：-1
 

提示：
1 <= nums.length <= 5000
-104 <= nums[i] <= 104
nums 中的每个值都 独一无二
题目数据保证 nums 在预先未知的某个下标上进行了旋转
-104 <= target <= 104

*/

#[allow(unused)]
pub struct Solution;


#[allow(unused)]
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l <= r {
            let mid = (l + r) >> 1;
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[l] <= nums[mid] {
                if target < nums[mid] && target >= nums[l] {
                    r = mid - 1;
                }else {
                    l = mid + 1;
                }
            }else {
                if target > nums[mid] && target <= nums[r] {
                    l = mid + 1;
                }else {
                    r = mid - 1;
                }
            }
        }
        -1
    }
}


#[cfg(test)]
mod test {
    use  super::*;

    #[test]
    fn test_search() {
        let l = vec![4,5,6,7,0,1,2];
        let r = Solution::search(l, 0);
        assert_eq!(r, 4);
    }

    #[test]
    fn test_search2() {
        let l = vec![4,5,6,7,0,1,2];
        let r = Solution::search(l, 3);
        assert_eq!(r, -1);
    }

    #[test]
    fn test_search3() {
        let l = vec![1];
        let r = Solution::search(l, 0);
        assert_eq!(r, -1);
    }

    #[test]
    fn test_search4() {
        let l = vec![3, 5, 1];
        let r = Solution::search(l, 3);
        assert_eq!(r, 0);
    }

    #[test]
    fn test_search5() {
        let l = vec![1];
        let r = Solution::search(l, 2);
        assert_eq!(r, -1);
    }

    #[test]
    fn test_search6() {
        let l = vec![1, 3];
        let r = Solution::search(l, 2);
        assert_eq!(r, -1);
    }

}