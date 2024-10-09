use std::collections::HashMap;

#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut res = Vec::new();
        if digits.is_empty() {
            return res;
        }
        let mut map = HashMap::new();
        map.insert('2', String::from("abc"));
        map.insert('3', String::from("def"));
        map.insert('4', String::from("ghi"));
        map.insert('5', String::from("jkl"));
        map.insert('6', String::from("mno"));
        map.insert('7', String::from("pqrs"));
        map.insert('8', String::from("tuv"));
        map.insert('9', String::from("wxyz"));

        Solution::dfs(0, &mut String::new(), &digits.chars().collect(), &mut res, &map);
        res
    }

    fn dfs(i: usize, s: &mut String, digits: &Vec<char>, res: &mut Vec<String>, map: &HashMap<char, String>) {
        if s.len() == digits.len() {
            res.push(s.clone());
            return ;
        }

        for c in map.get(&digits[i]).unwrap().chars() {
            s.push(c);
            Solution::dfs(i + 1, s, digits, res, map);
            s.pop();
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_letter_combinations() {
        let r = Solution::letter_combinations(String::from("23"));
        assert_eq!(r, vec!["ad","ae","af","bd","be","bf","cd","ce","cf"]);
    }
}