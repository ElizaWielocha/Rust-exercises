// Write a function to find the longest common prefix string amongst an array of strings.
// If there is no common prefix, return an empty string "".


impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let first = &strs[0];
        // checking from the longest prefix (=first string from list) 
        // and decresing it by 1 till it fit in all strings in list
        for i in (0..first.len()).rev() {
            let prefix = &first[0..=i];                     
            if strs.iter().all(|s| s.starts_with(prefix)) { // if all strings in list starts with this prefix
                return prefix.to_string();
            }
        }
        String::from("")                                    // fits with return type
    }
}