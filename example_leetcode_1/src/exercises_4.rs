/// 4.
///
/// 给定两个大小为 m 和 n 的有序数组 nums1 和 nums2。
///
/// 请你找出这两个有序数组的中位数，并且要求算法的时间复杂度为 O(log(m + n))。
///
/// 你可以假设 nums1 和 nums2 不会同时为空。
///
/// 示例 1:
///
/// nums1 = [1, 3]
/// nums2 = [2]
///
/// 则中位数是 2.0
/// 示例 2:
///
/// nums1 = [1, 2]
/// nums2 = [3, 4]
///
/// 则中位数是 (2 + 3)/2 = 2.5
///
/// 来源：力扣（LeetCode）
/// 链接：https://leetcode-cn.com/problems/median-of-two-sorted-arrays
/// 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays_1(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut n1_len, mut n2_len) = (nums1.len(), nums2.len());
        let (mut nums1_mut, mut nums2_mut) = (nums1, nums2);
        if n1_len > n2_len {
            let re = nums1_mut;
            nums1_mut = nums2_mut;
            nums2_mut = re;
            let re = n1_len;
            n1_len = n2_len;
            n2_len = re;
        }
        let (mut min, mut max) = (0, n1_len);
        while min <= max {
            let i = (min + max) / 2;
            let j = (n1_len + n2_len + 1) / 2 - i;
            if j != 0 && i != n1_len && (i < max) && (nums2_mut[j - 1] > nums1_mut[i]) {
                min = i + 1;
            } else if i != 0 && j != n2_len && (i > min) && (nums1_mut[i - 1] > nums2_mut[j]) {
                max = i - 1;
            } else {
                let mut max_left = 0.0;

                if i == 0 {
                    max_left = nums2_mut[j - 1] as f64;
                } else if j == 0 {
                    max_left = nums1_mut[i - 1] as f64;
                } else {
                    max_left = nums1_mut[i - 1].max(nums2_mut[j - 1]) as f64
                }
                if ((n1_len + n2_len) % 2) == 1 {
                    return max_left as f64;
                }

                let mut min_right: f64 = 0.0;

                if i == n1_len {
                    min_right = nums2_mut[j] as f64;
                } else if j == n2_len {
                    min_right = nums1_mut[i] as f64
                } else {
                    min_right = nums1_mut[i].min(nums2_mut[j]) as f64;
                }
                return (max_left + min_right) / 2.0;
            }
        }
        0.0
    }

    ///
    /// 标准库实现
    ///
    pub fn find_median_sorted_arrays_2( nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut nums1_mut, mut nums2_mut) = (nums1, nums2);
        fn get_median(v: Vec<i32>) -> f64 {
            let len = v.len();
            if len % 2 == 0 {
                let m_lhs = (len / 2) as usize - 1;
                (v[m_lhs] + v[m_lhs + 1]) as f64 / 2 as f64
            } else {
                let m = ((len - 1) / 2) as usize;
                v[m] as f64
            }
        }

        if nums1_mut.len() == 0 {
            get_median(nums2_mut)
        } else if nums2_mut.len() == 0 {
            get_median(nums1_mut)
        } else {
            let mut nums_concat = vec![];
            nums_concat.extend_from_slice(&nums1_mut);
            nums_concat.extend_from_slice(&nums2_mut);
            nums_concat.sort_by(|x, y| x.cmp(y));
            get_median(nums_concat)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_median_sorted_arrays() {
        let res = Solution::find_median_sorted_arrays_1([1, 2].to_vec(), [3, 4].to_vec());
        assert_eq!(res, 2.5);
        let res = Solution::find_median_sorted_arrays_2([1, 2].to_vec(), [3].to_vec());
        assert_eq!(res, 2 as f64);
    }
}