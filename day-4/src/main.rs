use std::fs;

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

fn main() {
    let filename = "src/in";
    let content = fs::read_to_string(filename).expect("Should have been able to read the file");

    let arr: Vec<Vec<char>> = content
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let mut ans = 0;
    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            // println!("Itre: x = {}, y = {}", i, j);
            if test(&arr, i, j, 1, 0) {
                ans += 1;
            }
            if test(&arr, i, j, 0, 1) {
                ans += 1;
            }
            if test(&arr, i, j, 1, 1) {
                ans += 1;
            }
            if test(&arr, i, j, -1, 0) {
                ans += 1;
            }
            if test(&arr, i, j, 0, -1) {
                ans += 1;
            }
            if test(&arr, i, j, -1, -1) {
                ans += 1;
            }
            if test(&arr, i, j, 1, -1) {
                ans += 1;
            }
            if test(&arr, i, j, -1, 1) {
                ans += 1;
            }
        }
    }
    println!("Ans: {}.", ans);
}
