// Given an integer array nums sorted in non-decreasing order, 
// remove the duplicates in-place such that each unique element appears only once. 
// The relative order of the elements should be kept the same. 
// Then return the number of unique elements in nums.

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.sort();                    // sort list
        nums.dedup();                   // remove duplicates from sorted list
        return nums.len() as i32;       // return number of elements in the list
    }
}