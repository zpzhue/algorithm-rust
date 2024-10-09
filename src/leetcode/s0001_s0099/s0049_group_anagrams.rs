use std::collections::HashMap;

#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs {
            let key = {
                let mut arr = s.chars().collect::<Vec<char>>();
                arr.sort();
                arr.iter().collect::<String>()
            };
            let val = map.entry(key).or_insert(Vec::new());
            val.push(s);
        }
        map.into_iter().map(|(_, v)| v).collect()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_group_anagrams1(){
        let v = vec![String::from("eat"), String::from("tea"), String::from("tan"), String::from("ate"), String::from("nat"), String::from("bat")];
        let r = Solution::group_anagrams(v);    
        println!("function group_anagrams result is: {:?}", r);
        assert_eq!(r, vec![vec![String::from("bat")], vec![String::from("nat"), String::from("tan")], vec![String::from("ate"), String::from("eat"), String::from("tea")]]);
    }
}