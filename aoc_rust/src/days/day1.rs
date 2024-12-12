use std::collections::HashMap;
use itertools::Itertools;

#[allow(dead_code)]
pub fn part_one() -> i64 {

    let s: &str = include_str!("../../input/day1.txt");

    let parts: std::str::Split<'_, &str> = s.split("\n");

    let mut v: Vec<String> = Vec::new();

    for x in parts {
        v.push(x.replace("\r", ""));
    }

    let mut left: Vec<String> = Vec::new();
    let mut right: Vec<String> = Vec::new();

    for x in v {
        if let Some((l, r)) = x.split_once("   ") { // Use split_once for exactly two parts
            left.push(l.to_string());
            right.push(r.to_string());
        } else {
            println!("Skipping invalid input: {}", x); // Handle invalid input gracefully
        }
    }

    left.sort();
    right.sort();

    let mut acc: i64 = 0;    

    for (i,_) in left.iter().enumerate() {
        acc += (left[i].parse::<i64>().unwrap_or(0) - right[i].parse::<i64>().unwrap_or(0)).abs();
    }
    acc
}

#[allow(dead_code)]
pub fn part_two() -> usize {

    let input: &str = include_str!("../../input/day1.txt");

    let pairs = input.lines().map(|line| {
        let (num1, num2) = line.split_once("   ").unwrap();
        let num1: usize = num1.parse().unwrap();
        let num2: usize = num2.parse().unwrap();

        (num1, num2)
    });

    let (list1, list2): (Vec<_>, Vec<_>) = pairs.unzip();

    let mut count1 = HashMap::<usize,usize>::new();
    let mut count2 = HashMap::<usize,usize>::new();

    for x in list1 {
        let l1 = count1.entry(x).or_default();
        *l1 += 1;
    }
    for x in list2 {
        let l2 = count2.entry(x).or_default();
        *l2 += 1;
    }

    let mut result = 0;
    
    for x in count1 {
        if count2.contains_key(&x.0) {
            result += x.0 * x.1 * *count2.get(&x.0).unwrap();
        }
    }
    
    result
}

#[allow(dead_code)]
pub fn part_one_reddit() -> usize {
    let input: &str = include_str!("../../input/day1.txt");

    let pairs = input.lines().map(|line| {
        let (num1, num2) = line.split_once("   ").unwrap();
        let num1: usize = num1.parse().unwrap();
        let num2: usize = num2.parse().unwrap();

        (num1, num2)
    });

    let (mut list1, mut list2): (Vec<_>, Vec<_>) = pairs.unzip();

    list1.sort_unstable();
    list2.sort_unstable();

    let mut result = 0;
    for (n1, n2) in list1.into_iter().zip(list2.into_iter()) {
        result += n1.abs_diff(n2);
    }

    result
}

#[allow(dead_code)]
pub fn part_two_reddit() -> usize {
    let input = include_str!("../../input/day1.txt");

    let pairs = input.lines().map(|line| {
        let (num1, num2) = line.split_once("   ").unwrap();
        let num1: usize = num1.parse().unwrap();
        let num2: usize = num2.parse().unwrap();

        (num1, num2)
    });

    let (list1, list2): (Vec<_>, Vec<_>) = pairs.unzip();

    let counts = list2.into_iter().counts();

    let result = list1
        .into_iter()
        .map(|n| counts.get(&n).unwrap_or(&0) * n)
        .sum();

    result
}
