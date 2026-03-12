struct Solution {}

impl Solution {
    fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.is_empty() && nums2.is_empty() {
            panic!("Precondition failed!")
        }

        let n_left = (nums1.len() + nums2.len()).div_ceil(2);
        let mut i1lower = if n_left >= nums2.len() { n_left - nums2.len() } else { 0 };
        let mut i1upper = std::cmp::min(n_left, nums1.len());
        let is_even = (nums1.len() + nums2.len()).is_multiple_of(2);

        loop {
            let i1mid = i1lower + ((i1upper - i1lower) / 2);
            let i2mid = n_left - i1mid;

            let l1 = if i1mid > 0 { nums1[i1mid - 1] } else { i32::MIN };
            let l2 = if i2mid > 0 { nums2[i2mid - 1] } else { i32::MIN };
            let r1 = if i1mid < nums1.len() { nums1[i1mid] } else { i32::MAX };
            let r2 = if i2mid < nums2.len() { nums2[i2mid] } else { i32::MAX };

            let l = std::cmp::max(l1, l2);
            let r = std::cmp::min(r1, r2);
            if l <= r {
                if is_even {
                    return (l as f64 + r as f64) / 2.0;
                } else {
                    return l as f64;
                }
            }
            if l1 < l2 {
                i1lower = i1mid + 1;
            } else {
                i1upper = i1mid - 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_median_sorted_arrays() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.0);

        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.5);
    }
}
