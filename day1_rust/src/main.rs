
pub fn part_1() -> i64 {
    let input: &str = include_str!("../../input/day1.txt");

    let parts: std::str::Split<'_, &str> = s_input.split("\n");

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

fn main() {
    
    let result = part_1();
    print!("{}", result)
}
