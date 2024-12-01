use std::fs;

fn main() {
    println!("Hello.\n");
    let filename = "in";
    let content = fs::read_to_string(filename).expect("Should have been able to read the file");

    // let mut a: Vec<i32> = vec![];
    // let mut b: Vec<i32> = vec![];
    let mut a: Vec<i32> = content
        .split("\n")
        .map(|line| {
            let x = line.split_once("   ").unwrap();
            x.0.trim().parse::<i32>().unwrap()
        })
        .collect();
    let mut b: Vec<i32> = content
        .split("\n")
        .map(|line| {
            let x = line.split_once("   ").unwrap();
            x.1.trim().parse::<i32>().unwrap()
        })
        .collect();
    a.sort();
    b.sort();
    let mut ans = 0;
    for i in 0..a.len() {
        ans = ans + (a[i] - b[i]).abs();
        // println!(
        //     "Ans itr {}, a[i] = {}, b[i] = {}, ans = {}",
        //     i, a[i], b[i], ans
        // );
    }
    println!("Ans: {}.", ans);
}
