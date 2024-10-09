/**
 *
168. Excel表列名称
简单
相关标签
相关企业
给你一个整数 columnNumber ，返回它在 Excel 表中相对应的列名称。

例如：

A -> 1
B -> 2
C -> 3
...
Z -> 26
AA -> 27
AB -> 28
...


示例 1：

输入：columnNumber = 1
输出："A"
示例 2：

输入：columnNumber = 28
输出："AB"
示例 3：

输入：columnNumber = 701
输出："ZY"
示例 4：

输入：columnNumber = 2147483647
输出："FXSHRXW"


提示：

1 <= columnNumber <= 2^31 - 1
 *
 */

#[allow(unused)]
pub struct Solution {}


impl Solution {
    pub fn convert_to_title(column_number: i32) -> i32 {
       println!("{}", 'A' as u8);
       column_number 
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_168() {
        assert_eq!(Solution::convert_to_title(1), 1);
        assert_eq!(Solution::convert_to_title(28), 28);
    }
}
