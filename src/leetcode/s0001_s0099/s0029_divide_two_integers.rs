#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == 0{ return 0; }
        if divisor == 1 { return dividend; }
        if divisor == -1 {
            if dividend > i32::MIN {
                return -dividend;
            }else {
                return i32::MAX
            }
        };

        let mut a = i64::from(dividend);
        let mut b = i64::from(divisor);
        let mut sign = 1_i64;
        if (a < 0 && b > 0) || (a > 0 && b < 0){
            sign = -1
        }

        if a < 0 {
            a = -a
        }
        if b < 0 {
            b = -b
        }

        let res = Solution::div(a, b);
        if sign < 0 {
            return -res as i32;
        }else {
            return res as i32;
        }
    }

    fn div(a: i64, b: i64) -> i64 {
        if a < b {
            return 0;
        }
        let mut count = 1_i64;
        let mut tb = b;
        while (tb + tb) < a {
            count = count + count;
            tb = tb + tb;
        }
        return count + Solution::div(a - tb, b);
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_divide1(){
        let r = Solution::divide(10, 3);
        println!("function divide result is: {}", r);
        assert_eq!(r, 3);
    }

    #[test]
    fn test_divide2(){
        let r = Solution::divide(7, -3);
        println!("function divide result is: {}", r);
        assert_eq!(r, -2);
    }
}