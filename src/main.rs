mod ownership;

fn main() {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n == 0 {
            0
        } else if n == 1 {
            1
        } else {
            let mut array: [i32; 101] = [0; 101];
            array[0] = 0;
            array[1] = 1;
            for i in 2..(n as usize + 1) {
                if i % 2 == 0 {
                    array[i] = array[(i/2)];
                } else {
                    array[i] = array[(i/2)] + array[(i/2 + 1)]
                }
            }
            *array.iter().max().unwrap_or(&0)
        }
    }

    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        use std::cmp::Reverse;
        let mut num_clone = nums.clone();
        num_clone.sort_by_key(|w| Reverse(*w));
        num_clone[(k - 1) as usize]
    }

    println!("{}", get_maximum_generated(7))
}
