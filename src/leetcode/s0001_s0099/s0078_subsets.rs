
#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut track: Vec<i32> = vec![];
        Solution::backtrack(&nums, &mut track, 0, &mut res);
        res
    }

    fn backtrack(nums: &Vec<i32>, track: &mut Vec<i32>, index: usize, res: &mut Vec<Vec<i32>>) {
        res.push(track.clone());
        for i in index..nums.len() {
            track.push(nums[i]);
            Solution::backtrack(nums, track, i + 1, res);
            track.pop();
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_subsets(){
        let r = Solution::subsets(vec![1, 2, 3]);
        println!("{:?}", r);
    }
}