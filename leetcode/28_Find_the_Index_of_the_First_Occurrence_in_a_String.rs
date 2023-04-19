// Given two strings needle and haystack, 
// return the index of the first occurrence of needle in haystack, 
// or -1 if needle is not part of haystack.

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
      // lengths of haystack and needle
      let haystackL = haystack.len();
      let needleL = needle.len();

      // base cases
      if needleL == 0 {
        return 0
      }
      else {
        if haystackL < needleL {
          return -1
        }
      }

      for i in 0..=(haystackL - needleL) {
          if &haystack[i..i+needleL] == needle { // if substring from haystack == needle
              return i as i32
          }
      }

      return -1
    }
}