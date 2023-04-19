// Given an integer x, return true if x is a palindrome, and false otherwise.

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let stringX = x.to_string();
        if stringX.chars().rev().collect::<String>() == stringX {
            return true;
        } else {
            return false
        }
    }
}