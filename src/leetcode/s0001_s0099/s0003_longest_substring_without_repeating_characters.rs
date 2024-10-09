use std::collections::HashMap;

#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map:HashMap<char, usize> = HashMap::new();
        let mut start_index = -1_i32;
        let mut longest_size = 0;
        for (index, sc) in s.chars().enumerate() {
            let item = map.get(&sc);
            if item.is_some() && *item.unwrap() as i32 > start_index {
                start_index = *item.unwrap() as i32;
                map.insert(sc, index);
            }else {
                map.insert(sc, index);
                if index as i32 - start_index > longest_size {
                    longest_size = index as i32 - start_index
                }
            }
        }
        longest_size as i32
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_length_of_longest_substring1(){
        let s = String::from("abcabcbb");
        let r = Solution::length_of_longest_substring(s);
        println!("function length_of_longest_substring result is: {}", r);
        assert_eq!(r, 3);
    }

    #[test]
    fn test_length_of_longest_substring2(){
        let s = String::from("bbbbb");
        let r = Solution::length_of_longest_substring(s);
        println!("function length_of_longest_substring result is: {}", r);
        assert_eq!(r, 1);
    }

    #[test]
    fn test_length_of_longest_substring3(){
        let s = String::from("pwwkew");
        let r = Solution::length_of_longest_substring(s);
        println!("function length_of_longest_substring result is: {}", r);
        assert_eq!(r, 3);
    }

    #[test]
    fn test_length_of_longest_substring4(){
        let s = String::from(" ");
        let r = Solution::length_of_longest_substring(s);
        println!("function length_of_longest_substring result is: {}", r);
        assert_eq!(r, 1);
    }

    #[test]
    fn test_length_of_longest_substring5(){
        let s = String::from("dvdf");
        let r = Solution::length_of_longest_substring(s);
        println!("function length_of_longest_substring result is: {}", r);
        assert_eq!(r, 3);
    }

}
