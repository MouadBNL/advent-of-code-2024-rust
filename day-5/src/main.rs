use std::{cmp::Ordering, fs};

fn read_input() -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let filename = "src/in";
    let content = fs::read_to_string(filename).expect("Should have been able to read the file");
    let lines: Vec<String> = content
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let mut order: Vec<(i32, i32)> = vec![];
    let mut updates: Vec<Vec<i32>> = vec![];
    for i in 0..lines.len() {
        let line = lines[i].trim();
        if line.len() == 5 {
            let nums = line
                .split_once("|")
                .map(|l| (l.0.parse::<i32>().unwrap(), l.1.parse::<i32>().unwrap()))
                .unwrap();
            order.push(nums);
        } else if line.len() > 5 {
            updates.push(line.split(",").map(|x| x.parse::<i32>().unwrap()).collect());
        }
    }

    (order, updates)
}

fn check_input(input: &(Vec<(i32, i32)>, Vec<Vec<i32>>)) {
    let (order, updates) = input;
    println!("Rules:");
    for rule in order {
        println!("Rule: {} before {}", rule.0, rule.1)
    }
    println!("Updates:");
    for update in updates {
        print!("Arr: ");
        for i in update {
            print!("{} ", i);
        }
        println!();
    }
}

fn main() {
    let (_order, _updates) = read_input();
    let order = _order.clone();
    let updates = _updates.clone();

    let mut ans = 0;
    let mut ans2 = 0;
    for mut arr in updates {
        let mut good = true;
        for i in 0..arr.len() {
            let x = arr[i];
            for rule in &order {
                if rule.1 == x {
                    for j in i..arr.len() {
                        if rule.0 == arr[j] {
                            good = false;
                            break;
                        }
                    }
                }
                if !good {
                    break;
                }
            }
            if !good {
                break;
            }
        }
        if good {
            ans += arr[arr.len() / 2];
        } else {
            arr.sort_by(|a, b| {
                for rule in &order {
                    if rule.0 == a.abs() && rule.1 == b.abs() {
                        return Ordering::Less;
                    }
                    if rule.1 == a.abs() && rule.0 == b.abs() {
                        return Ordering::Greater;
                    }
                }

                return Ordering::Equal;
            });

            print!("Arr: ");
            for x in &arr {
                print!("{} ", x);
            }
            println!("");
            ans2 += arr[arr.len() / 2];
            // wrong_updates.push(arr);
        }
    }

    println!("Ans: {}", ans2);

    // check_input(order, updates);
}
