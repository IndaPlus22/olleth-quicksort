use std::io::{self, BufRead, Read};
use rand::{thread_rng, Rng};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

const INSERTION_SORT_THRESHOLD: usize = 100;

fn quicksort<T: Ord>(arr: &mut [T], len: usize) {
    if len <= 1 {
        return;
    }
    if len <= INSERTION_SORT_THRESHOLD {
        insertion_sort(arr, len);
        return;
    }
    let (pivot, pattern_detected) = choose_pivot(arr, len);
    let mid = partition(arr, pivot, len);
    if pattern_detected {
        return;
    }
    quicksort(&mut arr[..mid], len);
    quicksort(&mut arr[mid + 1..], len);
}

fn insertion_sort<T: Ord>(arr: &mut [T], len: usize) {
    for i in 1..len {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

//lomuto partition
fn partition<T: Ord>(arr: &mut [T], pivot: usize, len: usize) -> usize {
    let last = len - 1;
    arr.swap(pivot, last);
    let mut i = 0;
    for j in 0..last {
        if arr[j] <= arr[last] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, last);
    i
}

//Column of 5 method to guarentee O(nlog(n)) runtime
fn choose_pivot<T: Ord>(arr: &[T], len: usize) -> (usize, bool) {
    let mut rng = thread_rng();
    let mut indices = [0; 5];
    for i in 0..5 {
        indices[i] = rng.gen_range(0..len);
    }
    indices.sort_unstable();
    let median_index = indices[2];

    let (pivot, pattern_detected) = if samenumber_pattern(arr, median_index) {
        (len / 2, true)
    } else {
        (median_index, false)
    };
    (pivot, pattern_detected)
}

//Checks if arr as same numbers e.g. [0, 1, 1, 2, 2, 1, 5, 3, 9, 6].
fn samenumber_pattern<T: Ord>(arr: &[T], pivot: usize) -> bool {
    let mut i = pivot;
    let mut j = pivot + 1;
    while i > 0 && arr[i] == arr[i - 1] {
        i -= 1;
    }
    while j < arr.len() && arr[j] == arr[j - 1] {
        j += 1;
    }
    j - i >= arr.len() / 2
}

fn main() {
    random_arr_generation();
    let length = test().len();
    let mut arr = test();
    
    quicksort(&mut arr, length);
    
    // output
    for _v in arr {
        print!("{} ", _v);
    }
    println!("");
}

// thank you jblomlof for letting me know how this kattis deals with input
fn input() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input);

    let mut vals: Vec<i32> = input
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    vals
}

fn test() -> Vec<i32> {
    let input = std::fs::read_to_string("src/tests.txt").unwrap();

    let mut vals: Vec<i32> = input
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    // println!("beg: {:?}", vals);
    vals
}

fn random_arr_generation() {
    let mut file = OpenOptions::new().write(true).open("src/tests.txt").unwrap();
    let mut rng = rand::thread_rng();
  
    let random_vec: Vec<i32> = (0..50).map(|_| rng.gen_range(-10000..=10000)).collect();
        
    // Write the vector elements to the file
    for number in &random_vec {
        write!(file, "{} ", number).expect("Unable to write data to file");
    }

}