use std::fs;

fn read_input() -> Vec<(i64, Vec<i64>)> {
    let filename = "src/in";
    let content = fs::read_to_string(filename).expect("Should have been able to read the file");
    let lines: Vec<(i64, Vec<i64>)> = content
        .split("\n")
        .map(|line| {
            let parts = line.split_once(":").unwrap();
            let sum = parts.0.trim().parse::<i64>().unwrap();
            let nums = parts
                .1
                .trim()
                .split(" ")
                .map(|n| n.trim().parse::<i64>().unwrap())
                .collect();
            (sum, nums)
        })
        .collect();
    lines
}

fn concat(a: i64, b: i64, sum: i64) -> i64 {
    let str = a.to_string() + &b.to_string();
    if str.len() > sum.to_string().len() {
        return -1;
    };
    return str.parse::<i64>().unwrap();
}

fn recursive_test(equation: &(i64, Vec<i64>), i: usize, sum: i64) -> bool {
    if sum > equation.0 {
        return false;
    }
    if i >= equation.1.len() {
        // println!("Sum: {}, Expected: {}", sum, equation.0);
        return sum == equation.0;
    }

    let res = recursive_test(equation, i + 1, sum + equation.1[i])
        || recursive_test(equation, i + 1, sum * equation.1[i]);
    if res {
        return res;
    } else {
        let c = concat(sum, equation.1[i], equation.0);
        if c == -1 {
            return false;
        }
        return recursive_test(equation, i + 1, c);
    }
}

fn main() {
    let equations = read_input();
    let mut ans = 0;
    for eq in equations {
        print!("{} = ", eq.0);
        for i in &eq.1 {
            print!("{} # ", i);
        }
        if recursive_test(&eq, 1, eq.1[0]) {
            ans += eq.0;
            print!(" Calibrated");
        }
        println!("");
    }

    println!("Ans: {}.", ans);
}
