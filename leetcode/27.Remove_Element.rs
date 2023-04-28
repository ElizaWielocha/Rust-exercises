// Given an integer array nums and an integer val, 
// remove all occurrences of val in nums in-place. 
// The order of the elements may be changed. 
// Then return the number of elements in nums which are not equal to val.

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&item| item != val); //keep only elements that are not val
        // Retains only the elements specified by the predicate, passing a mutable reference to it.
        return nums.len() as i32;
    }
}