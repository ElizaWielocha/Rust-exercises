// Given a string s consisting of words and spaces, 
// return the length of the last word in the string.
// A word is a maximal substring consisting of non-space characters only.

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let last = s.split_whitespace().last().unwrap();
        return last.len() as i32;
    }
}