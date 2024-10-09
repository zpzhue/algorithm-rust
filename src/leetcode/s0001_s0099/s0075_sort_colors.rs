
#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut start, mut end) = (-1, nums.len());
        let mut index = 0_usize;
        let swap = |nums: &mut Vec<i32>, k: usize, l: usize| {
            if k == l {
                return ;
            }
            nums[k] ^= nums[l];
            nums[l] ^= nums[k];
            nums[k] ^= nums[l];
        };

        while index < end {
            if nums[index] == 0 {
                start = start + 1;
                swap(nums, start as usize, index);
                index += 1;
            }else if nums[index] == 1 {
                index += 1;
            }else {
                end -= 1;
                swap(nums, end, index);
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort_colors1(){
        let mut s = vec![2,0,2,1,1,0];
        Solution::sort_colors(&mut s);
        assert_eq!(s, [0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_sort_colors2(){
        let mut s = vec![2];
        Solution::sort_colors(&mut s);
        assert_eq!(s, [2]);
    }

    #[test]
    fn test_sort_colors3(){
        let mut s = vec![1, 2, 0];
        Solution::sort_colors(&mut s);
        assert_eq!(s, [0, 1, 2]);
    }

    #[test]
    fn test_sort_colors4(){
        let mut s = vec![0, 1, 0];
        Solution::sort_colors(&mut s);
        assert_eq!(s, [0, 0, 1]);
    }
}