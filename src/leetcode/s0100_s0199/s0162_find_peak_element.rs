
#[allow(unused)]
pub struct Solution;


#[allow(unused)]
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0_usize, nums.len() - 1);
        while left < right {
            let mid = (left + right) >> 1;
            if nums[mid] > nums[mid + 1] {
                right = mid
            }else {
                left = mid + 1
            }
        }
        left as i32
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_peak_element(){
        let nums = vec![1,2,3,1];
        let r = Solution::find_peak_element(nums);
        assert_eq!(r, 2);
    }

    #[test]
    fn test_find_peak_element2(){
        let nums = vec![1,2,1,3,5,6,4];
        let r = Solution::find_peak_element(nums);
        // assert_eq!(r, 1);
        assert_eq!(r, 5);
    }
}