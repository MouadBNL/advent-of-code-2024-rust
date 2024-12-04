use std::{env, fs};

fn test(arr: &Vec<Vec<char>>, x: usize, y: usize, i: i32, j: i32) -> bool {
    // println!("x = {}, y = {}, i = {}, j = {}", x, y, i, j);
    // println!("(x, {}) as i32 + (i, {}) * 3 = {}", x, i, x as i32 + i * 3);
    // println!("(y, {}) as i32 + (j, {}) * 3 = {}", y, j, y as i32 + j * 3);
    if x as i32 + i * 3 < 0
        || x as i32 + i * 3 >= arr.len() as i32
        || y as i32 + j * 3 < 0
        || y as i32 + j * 3 >= arr[x].len() as i32
    {
        return false;
    }
    if arr[x][y] == 'X'
        && arr[(x as i32 + i) as usize][(y as i32 + j) as usize] == 'M'
        && arr[(x as i32 + i * 2) as usize][(y as i32 + j * 2) as usize] == 'A'
        && arr[(x as i32 + i * 3) as usize][(y as i32 + j * 3) as usize] == 'S'
    {
        return true;
    }
    return false;
}

fn testX(arr: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if (arr[x][y] != 'A') {
        return false;
    }
    println!("x+1 = {}", x + 1);
    println!("y+1 = {}", y + 1);
    if !(arr[x - 1][y - 1] == 'M' && arr[x + 1][y + 1] == 'S'
        || arr[x - 1][y - 1] == 'S' && arr[x + 1][y + 1] == 'M')
    {
        return false;
    }
    if !(arr[x + 1][y - 1] == 'M' && arr[x - 1][y + 1] == 'S'
        || arr[x + 1][y - 1] == 'S' && arr[x - 1][y + 1] == 'M')
    {
        return false;
    }
    return true;
}

fn main() {
    let filename = "src/in";
    let content = fs::read_to_string(filename).expect("Should have been able to read the file");

    let arr: Vec<Vec<char>> = content
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let mut ans = 0;
    for i in 1..(arr.len() - 1) {
        for j in 1..(arr[i].len() - 1) {
            println!(
                "Itre: x = {}, y = {}, arr.len = {}, arr[i].len() = {}",
                i,
                j,
                arr.len(),
                arr[i].len()
            );
            if testX(&arr, i, j) {
                ans += 1;
            }
        }
    }
    println!("Ans: {}.", ans);
}
