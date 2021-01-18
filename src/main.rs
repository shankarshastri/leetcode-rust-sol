use std::collections::HashMap;
use std::cmp::min;

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

    pub fn count_vowel_strings(n: i32) -> i32 {
        let def_c = 1;

        let empty_vec: Vec<char> = Vec::new();

        fn foldFn(acc: i32, b: &char) -> i32 {
            acc + count(&vec![b.clone()])
        }

        fn count(c_v: &Vec<char>) -> i32 {
            let L1: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
            let L2: Vec<char> = vec!['e', 'i', 'o', 'u'];
            let L3: Vec<char> = vec!['i', 'o', 'u'];
            let L4: Vec<char> = vec!['o', 'u'];
            let L5: Vec<char> = vec!['u'];

            let len = c_v.len();

            if len == n { def_c }
            else if len == 0 {
                let k = L1.iter().fold(0,foldFn);
                k
            } else {
                match c_v.first() {
                    Some('a') => L1.iter().fold(0, foldFn),
                    Some('e') => L2.iter().fold(0, foldFn),
                    Some('i') => L3.iter().fold(0, foldFn),
                    Some('o') => L4.iter().fold(0, foldFn),
                    Some('u') => L5.iter().fold(0, foldFn),
                    _ => L1.iter().fold(0, foldFn),
                }
            }
        }

        count(&empty_vec)
    }

    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        use std::cmp::min;

        let mut m: HashMap<i32, i32> = HashMap::new();
        let mut res = 0;
        for e in nums {
            m.insert(e,  m.get(&e).unwrap_or(&0) + 1);
        }

        for (key,v) in &m {
            match m.get(&(k - key)) {
                Some(r) =>  {
                    res += min(v, r);
                }
                _ => {}
            }
        }
        res/2
    }

    println!("{}", get_maximum_generated(7))
}
