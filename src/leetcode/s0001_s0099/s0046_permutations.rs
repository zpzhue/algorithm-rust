
#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut used: Vec<bool> = vec![false; nums.len()];
        Solution::dfs(&nums, &mut Vec::new(), &mut used, &mut res);
        res
    }

    fn dfs(nums: &Vec<i32>, track: &mut Vec<i32>, used: &mut Vec<bool>, res: &mut Vec<Vec<i32>>){
        if track.len() == nums.len() {
            res.push(track.clone());
            return ;
        }
        for j in 0..nums.len() {
            if used[j] {
                continue;
            }
            track.push(nums[j]);
            used[j] = true;
            Solution::dfs(nums, track, used, res);
            track.pop();
            used[j] = false;
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_permute() {
        let r = Solution::permute(vec![1, 2, 3]);
        assert_eq!(r, vec![vec![1,2,3], vec![1,3,2], vec![2,1,3], vec![2,3,1], vec![3,1,2], vec![3,2,1]]);
    }
}