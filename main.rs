
// IMPLEMENT A FUNCTION THAT CHECKS WHETHER A GIVEN STRING IS A PALINDROME OR NOT

fn is_palindrome(s: &str) -> bool {
    let s_lower = s.to_lowercase();
    s_lower.chars().eq(s_lower.chars().rev())
}

fn main() {

    print!("**************************Q1*********************************\n");
    let string1 = "madam";
    let string2 = "hello"; 
    if is_palindrome(string1) {
        println!("'{}' is a palindrome.", string1);
    } else {
        println!("'{}' is not a palindrome.", string1);
    }
    if is_palindrome(string2) {
        println!("'{}' is a palindrome.", string2);
    } else {
        println!("'{}' is not a palindrome.", string2);
    }

    print!("****************************Q2*******************************\n");

    main2();

    print!("****************************Q3*******************************\n");

    main3();

    print!("****************************Q4*******************************\n");

    main4();

    print!("****************************Q5*******************************\n");

    main5();

    print!("****************************Q6*******************************\n");

    main6();
    
    print!("****************************Q7*******************************\n");

    main7();

    print!("*****************************Q8******************************\n");

    main8();

    print!("****************************Q9*******************************\n");

    main9();

    print!("****************************Q10******************************\n");

    main10();

    


}

// Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

fn main2() {
    let sorted_array = vec![1, 2, 3, 4, 4, 5, 6];
    let target = 4;
    match first_occurrence(&sorted_array, target) {
        Some(index) => println!("The first occurrence of {} is at index {}.", target, index),
        None => println!("The number {} is not in the array.", target),
    }
}

fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    if left < arr.len() && arr[left] == target {
        Some(left)
    } else {
        None
    }
}

//  Given a string of words, implement a function that returns the shortest word in the string.

fn main3() {
    let sentence = "Rust is a systems programming language focused on safety and performance";
    match shortest_word(sentence) {
        Some(word) => println!("The shortest word is '{}'.", word),
        None => println!("The input string is empty or contains no words."),
    }
}

fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}

//Implement a function that checks whether a given number is prime or not.

fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    

    for i in 2..=(num as f64).sqrt() as u64 {
        if num % i == 0 {
            return false; // If num is divisible by any number other than 1 and itself, it's not prime
        }
    }
    
    true 
}

fn main4() {
    let num = 17; // Define the number to be checked for primality

    // Call the 'is_prime' function to check if 'num' is prime
    if is_prime(num) {
        println!("{} is prime", num);
    } else {
        println!("{} is not prime", num);
    }
}

//Given a sorted array of integers, implement a function that returns the median of the array.

fn median(sorted_array: &[i32]) -> f64 {
    let len = sorted_array.len();
    if len % 2 == 1 {
        sorted_array[len / 2] as f64
    } else {
        let mid1 = sorted_array[len / 2 - 1];
        let mid2 = sorted_array[len / 2];
        (mid1 as f64 + mid2 as f64) / 2.0
    }
}

fn main5() {
    let sorted_array = vec![1, 3, 3, 6, 7, 8, 9];
    println!("The median is: {}", median(&sorted_array));
    
    let sorted_array_even = vec![1, 2, 3, 4, 5, 6, 8, 9];
    println!("The median is: {}", median(&sorted_array_even));
}

//Implement a function that finds the longest common prefix of a given set of strings.

fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let min_len = strs.iter().map(|s| s.len()).min().unwrap();
    let mut prefix = String::new();

    for i in 0..min_len {
        let current_char = strs[0].chars().nth(i).unwrap();
        for s in strs {
            if s.chars().nth(i).unwrap() != current_char {
                return prefix;
            }
        }
        prefix.push(current_char);
    }

    prefix
}

fn main6() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    println!("The longest common prefix is: {}", longest_common_prefix(&strings));

    let strings = vec![
        String::from("dog"),
        String::from("racecar"),
        String::from("car"),
    ];
    println!("The longest common prefix is: {}", longest_common_prefix(&strings));
}

//Implement a function that returns the kth smallest element in a given array.

fn kth_smallest(arr: &mut [i32], k: usize) -> Option<i32> {
    if k == 0 || k > arr.len() {
        return None; // k is out of valid range
    }
    arr.sort(); // Sort the array
    Some(arr[k - 1]) // Return the k-1th element
}

fn main7() {
    let mut arr = vec![7, 10, 4, 3, 20, 15];
    let k = 3;
    match kth_smallest(&mut arr, k) {
        Some(value) => println!("The {}-th smallest element is: {}", k, value),
        None => println!("Invalid value for k: {}", k),
    }

    let mut arr2 = vec![7, 10, 4, 3, 20, 15];
    let k2 = 6;
    match kth_smallest(&mut arr2, k2) {
        Some(value) => println!("The {}-th smallest element is: {}", k2, value),
        None => println!("Invalid value for k: {}", k2),
    }

    let mut arr3 = vec![7, 10, 4, 3, 20, 15];
    let k3 = 7; // Invalid since k3 is greater than the length of the array
    match kth_smallest(&mut arr3, k3) {
        Some(value) => println!("The {}-th smallest element is: {}", k3, value),
        None => println!("Invalid value for k: {}", k3),
    }
}

//Reverse a string in Rust

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main8() {
    let original = "hello, world!";
    let reversed = reverse_string(original);
    println!("Original: {}", original);
    println!("Reversed: {}", reversed);
}

//Merge two sorted arrays in Rust


fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(arr1.len() + arr2.len());
    let mut i = 0;
    let mut j = 0;

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }

    // Append remaining elements of arr1, if any
    while i < arr1.len() {
        merged.push(arr1[i]);
        i += 1;
    }

    // Append remaining elements of arr2, if any
    while j < arr2.len() {
        merged.push(arr2[j]);
        j += 1;
    }

    merged
}



fn main9() {
    let arr1 = vec![1, 3, 5, 7];
    let arr2 = vec![2, 4, 6, 8];
    let merged = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged array: {:?}", merged);
}

//Find the maximum subarray sum in Rust

fn max_subarray_sum(nums: &[i32]) -> i32 {

    let mut max_current = nums[0];
    let mut max_global = nums[0];

    for &num in &nums[1..] {
        max_current = std::cmp::max(num, max_current + num);
        if max_current > max_global {
            max_global = max_current;
        }
    }

    max_global
}

fn main10() {
    let arr = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("The maximum subarray sum is: {}", max_subarray_sum(&arr));

    let arr2 = vec![1, 2, 3, 4, 5];
    println!("The maximum subarray sum is: {}", max_subarray_sum(&arr2));

    let arr3 = vec![-1, -2, -3, -4];
    println!("The maximum subarray sum is: {}", max_subarray_sum(&arr3));
}




