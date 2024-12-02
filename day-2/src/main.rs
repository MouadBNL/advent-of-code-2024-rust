use std::fs;

fn is_safe(arr: &Vec<i32>) -> bool {
    if !arr.is_sorted_by(|a, b| a - b < 0) && !arr.is_sorted_by(|a, b| a - b > 0) {
        if !arr.is_sorted() {
            return false;
        }
    }
    for i in 1..arr.len() {
        let diff = (arr[i] - arr[i - 1]).abs();
        if diff > 3 || diff < 1 {
            return false;
        }
    }

    return true;
}

fn main() {
    let filename = "src/in";
    let content = fs::read_to_string(filename).expect("Should have been able to read the file");

    let mut ans = 0;
    for line in content.split("\n") {
        let arr: Vec<i32> = line
            .split(" ")
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();

        if is_safe(&arr) {
            ans += 1;
        } else {
            for i in 0..arr.len() {
                let mut test = arr.clone();
                test.remove(i);
                if is_safe(&test) {
                    ans += 1;
                    break;
                }
            }
        }
    }

    println!("Ans: {}.", ans);
}
