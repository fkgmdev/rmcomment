use std::fs;
use std::process;
use std::env::args;
fn main() {
    let args: Vec<String>  = args().collect();
    if args.len() == 1 || args.len() > 2 {
        println!("Usage: remcomment <filepath>");
        process::exit(1)
    }
    let file = String::from(&args[1]);
    let mut to_remove: Vec<i32> = Vec::new();
    let file_str = fs::read_to_string(&file).expect("couldnt read");
    if file_str.is_empty() {
        process::exit(1)
    }
    for (index, line) in file_str.lines().enumerate() {
        let mut remove = false;
        let chars: Vec<char> = line.chars().collect();
        for (cindex, char) in line.chars().enumerate() {
            if char == '/' && chars[cindex + 1] == '/' && chars[cindex + 3] != '*' {
                remove = true;
            }
        }
        if remove {
            to_remove.push(index as i32);
        }
    }
    let mut new_str = String::new();
    let lines: Vec<&str> = file_str.lines().collect();
    for index in 0..lines.len() {
        if !to_remove.contains(&(index as i32)) {
            new_str.push_str(&lines[index]);
            new_str.push_str("\n");
        }
    }
    fs::write(file, new_str).unwrap();
}
