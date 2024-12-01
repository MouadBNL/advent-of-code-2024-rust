use std::{collections::HashMap, fs};

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

    let freq = b.iter().copied().fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|f| *f += 1).or_insert(1);
        map
    });
    let mut ans1 = 0;
    let mut ans2 = 0;
    for i in 0..a.len() {
        ans1 += (a[i] - b[i]).abs();
        ans2 += a[i] * freq.get(&a[i]).unwrap_or(&0);
        // println!(
        //     "Ans itr {}, a[i] = {}, b[i] = {}, ans = {}",
        //     i, a[i], b[i], ans
        // );
    }
    println!("Part 1: {}.", ans1);
    println!("Part 2: {}.", ans2);
}
