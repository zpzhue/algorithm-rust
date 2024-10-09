use std::collections::HashMap;

#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map:HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for (i, v) in nums.iter().enumerate(){
            match map.get(&(target - v)) {
                Some(&x) => return vec![x, i as i32],
                None => map.insert(*v, i as i32),
            };
        }
        vec![]
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_two_sum(){
        let nums = vec![2,7,11,15];
        let target = 9;
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1])
    }
}