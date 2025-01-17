/**
    https://leetcode.cn/problems/jump-game/

55. 跳跃游戏
给定一个非负整数数组 nums ，你最初位于数组的 第一个下标 。
数组中的每个元素代表你在该位置可以跳跃的最大长度。
判断你是否能够到达最后一个下标。

 
示例 1：
输入：nums = [2,3,1,1,4]
输出：true
解释：可以先跳 1 步，从下标 0 到达下标 1, 然后再从下标 1 跳 3 步到达最后一个下标。

示例 2：
输入：nums = [3,2,1,0,4]
输出：false
解释：无论怎样，总会到达下标为 3 的位置。但该下标的最大跳跃长度是 0 ， 所以永远不可能到达最后一个下标。
 

提示：
1 <= nums.length <= 3 * 104
0 <= nums[i] <= 105

*/

#[allow(unused)]
pub struct Solution;


#[allow(unused)]
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut mx = 0;
        for i in 0..nums.len() {
            if i as i32 > mx {
                return false;
            }
            mx = mx.max(i as i32 + nums[i]);
        }
        true
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_jump1() {
        let nums = vec![2,3,1,1,4];
        let r = Solution::can_jump(nums);
        assert_eq!(r, true);
    }

    #[test]
    fn test_can_jump2() {
        let nums = vec![3,2,1,0,4];
        let r = Solution::can_jump(nums);
        assert_eq!(r, false);
    }
}