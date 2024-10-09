use std::collections::HashMap;

#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut max_count = 0;
        nums.into_iter().map(|v| {
            let val = map.get(&v).unwrap_or(&0) + 1;
            map.insert(v, val);
            max_count = max_count.max(val);
        }).collect::<()>();

        let mut k = k as usize;
        let mut res = vec![0; k];
        while k > 0 {
            let mut next = 0;
            for key in map.keys() {
                let val = map[key];
                if val == max_count {
                    res[k - 1] = *key;
                    k -= 1;
                }else if val < max_count {
                    next = next.max(val);
                }
            }
            max_count = next;
        }
        res
    }


    pub fn top_k_frequent2(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // use std::collections::BinaryHeap
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut res = vec![0; k as usize];
        nums.into_iter().map(|v| {
            let val = map.get(&v).unwrap_or(&0) + 1;
            map.insert(v, val);
        }).collect::<()>();

        let mut arr = map.iter().map(|(i, v)| (*i, *v)).collect::<Vec<(i32, i32)>>();
        arr.sort_by(|a ,b| a.1.cmp(&b.1) );
        let mut n = k as usize;
        while n > 0 {
            res[n - 1] = arr[arr.len() - n].0;
            n -= 1;
        }
        res
    }

    pub fn top_k_frequent3(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::BinaryHeap;

        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut res = vec![];
        nums.into_iter().map(|v| {
            let val = map.get(&v).unwrap_or(&0) + 1;
            map.insert(v, val);
        }).collect::<()>();

        let mut heap = BinaryHeap::new();

        for key in map.keys() {
            heap.push((*key, map[key]))
        }
        println!("heap: {:?}", heap);

        for i in 0.. k {
            println!("k={}, i={}", k, i);
            res.push(heap.pop().unwrap().0);
        }
        res
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_top_k_frequent1() {
        let s = vec![1,1,1,2,2,3];
        let r = Solution::top_k_frequent(s, 2);
        println!("{:?}", r);
    }
}