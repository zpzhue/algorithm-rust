
/**
69. x 的平方根 
给你一个非负整数 x ，计算并返回 x 的 算术平方根 。

由于返回类型是整数，结果只保留 整数部分 ，小数部分将被 舍去 。

注意：不允许使用任何内置指数函数和算符，例如 pow(x, 0.5) 或者 x ** 0.5 。

示例 1：
输入：x = 4
输出：2

示例 2：
输入：x = 8
输出：2
解释：8 的算术平方根是 2.82842..., 由于返回类型是整数，小数部分将被舍去。
 

提示：
0 <= x <= 231 - 1

通过次数713,497提交次数1,851,274

*/

#[allow(unused)]
pub struct Solution;


#[allow(unused)]
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x;
        }
        let mut l = 1;
        let mut r = x >> 1;

        while l < r {
            let mid = (l + r + 1) >> 1;
            if x / mid < mid {
                r = mid - 1
            }else {
                l = mid
            }
        }
        return l;
    }
}