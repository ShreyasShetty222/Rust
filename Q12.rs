//12. Find the maximum subarray sum in Rust

fn max_sub_array_sum(nums: &[i32]) -> i32 {
    let mut max_so_far = nums[0];
    let mut max_ending_here = nums[0];

    for &num in nums.iter().skip(1) {
        max_ending_here = i32::max(num, max_ending_here + num);
        max_so_far = i32::max(max_so_far, max_ending_here);
    }

    max_so_far
}

fn main() {
    let arr = vec![1, -3, 2, 1, -1];
    println!("{}", max_sub_array_sum(&arr)); // 3
}
