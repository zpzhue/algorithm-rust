
#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }
        let mut min = i32::MAX;
        let mut mid = i32::MAX;
        for n in nums {
            if n <= min {
                min = n;
            }else if n <= mid {
                mid = n
            }else {
                return true;
            }
        }
        false
    }
}