use std::io::{Read};


const INSERTION_SORT_THRESHOLD: usize = 101;

fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}





fn main() {

    let mut arr = input();
    qsort(&mut arr);
    
    // output
    for _v in arr {
        print!("{} ", _v);
    }
    println!("");
}

fn qsort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    if arr.len() < INSERTION_SORT_THRESHOLD {
        insertion_sort(arr);
        return;
    }

    if arr.len() > 0 {
        let pivot: usize = partition(&mut arr[..]);

        qsort(&mut arr[..(pivot + 1)]);
        qsort(&mut arr[(pivot + 1)..]);
    }
}

//hoere partition
fn partition(arr: &mut [i32]) -> usize {
    pivot(arr);
    
    let pivot = arr[0];
    let mut i = -1;
    let mut j = arr.len() as i32;

    loop {
        i += 1;
        while arr[i as usize] < pivot {
            i += 1;
        }

        j -= 1;
        while arr[j as usize] > pivot {
            j -= 1;
        }

        if i >= j {
            return j as usize;
        }

        arr.swap(i as usize, j as usize);
    }
}


//low, mid, high pivot selection
fn pivot(arr: &mut [i32]) {
    let b = arr.len() / 2;
    let c = arr.len() - 1;

    let mut median = 0;

    if arr[0] < arr[b] {
        if arr[b] < arr[c] {
            median = b;
        } else if arr[0] < arr[c] {
            median = c;
        }
    } else {
        if arr[c] < arr[b] {
            median = b;
        } else if arr[c] < arr[0] {
            median = c;
        }
    }

    arr.swap(median, 0);
}

//Throws run-time error for some reason?
fn median_of_medians(arr: &mut [i32]) -> i32 {
    if arr.len() <= 5 {
        arr.sort_unstable();
        return arr[arr.len() / 2];
    }

    let num_chunks = (arr.len() + 4) / 5;
    let mut medians = vec![0; num_chunks];

    for i in 0..num_chunks {
        let chunk_start = i * 5;
        let chunk_end = chunk_start + 5;
        let chunk_slice = &mut arr[chunk_start..chunk_end];
        medians[i] = median_of_medians(chunk_slice);
    }

    let pivot = median_of_medians(&mut medians);

    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        while arr[left] < pivot {
            left += 1;
        }
        while arr[right] > pivot {
            right -= 1;
        }
        if left <= right {
            arr.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    if left == right + 2 {
        arr.swap(right + 1, arr.len() / 2);
    } else if left == right + 1 {
        if arr[right] > pivot {
            arr.swap(right, arr.len() / 2);
        } else {
            arr.swap(left, arr.len() / 2);
        }
    } else {
        arr[arr.len() / 2] = pivot;
    }

    arr.swap(arr[arr.len() / 2] as usize, 0);
    return arr[arr.len() / 2];
}





fn input() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).expect("input failed");

    let vals: Vec<i32> = input
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    vals
}

