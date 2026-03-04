use std::usize;

struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut l1 = m - 1;
        let mut l2 = n - 1;
        for i in (0..(m + n) as usize).rev() {
            if (l1 >= 0 && l2 >= 0 && nums1[l1 as usize] >= nums2[l2 as usize]) || (l2 < 0) {
                nums1[i] = nums1[l1 as usize];
                l1 = l1 - 1;
            } else {
                nums1[i] = nums2[l2 as usize];
                l2 = l2 - 1;
            }
        }
    }
}
