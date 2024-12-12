use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn clean_input(s_input : String) -> (Vec<String>, Vec<String>){

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

    (left,right)
}

fn main() {
    // Create a path to the desired file
    let path: &Path = Path::new("input.txt");
    let display: std::path::Display<'_> = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file: File = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s: String = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("Loaded!",),
    }

    let (left,right) = clean_input(s);

    let mut acc: i64 = 0;    
    
    for (i,_) in left.iter().enumerate() {
        acc += (left[i].parse::<i64>().unwrap_or(0) - right[i].parse::<i64>().unwrap_or(0)).abs();
    }

    print!("{}",acc)

}
