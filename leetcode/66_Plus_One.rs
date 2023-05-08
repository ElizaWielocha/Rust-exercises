// You are given a large integer represented as an integer array digits, 
// where each digits[i] is the ith digit of the integer. 
// The digits are ordered from most significant to least significant in left-to-right order. 
// The large integer does not contain any leading 0's.

// Increment the large integer by one and return the resulting array of digits.

// EXAMPLES:
// Input: digits = [1,2,3]
// Output: [1,2,4]

// Input: digits = [9]
// Output: [1,0]


impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut d = digits;
        let mut last = d.len()-1;                   // index of the last element 

        for i in 0..d.len() { 
            if d[last] < 9 {                        // if the last element is less then 9
                d[last] = d[last] + 1;              // add 1 to it
                break
            }
            else if last == 0 && d[last] == 9 {     // if there is begining of the list and the last element is 9
                d[last] = 0;                        // the last element becomes 0
                d.insert(0, 1);                     // the first element becomes 1
                break
            }
            else if d[last] == 9 {                  // if the last element is 9
                d[last] = 0;                        // the last element becomes 0
            }

            last = last - 1;                        // decrement index of last element   

        }
        return d
    }
}
