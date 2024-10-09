mod leetcode;
mod utils;

fn main() {
    println!("Hello World&!");
    
    use leetcode::s0001_s0099::s0033_search_in_rotated_sorted_array::Solution;
    let l = vec![3, 5, 1];
    let r = Solution::search(l, 3);
    println!("{:?}", r);
}
