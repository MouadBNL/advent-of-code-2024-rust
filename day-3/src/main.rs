use std::fs;

fn parse_int(content: &Vec<char>, idx: usize) -> (u32, usize) {
    let mut nbr: u32 = 0;
    let mut len: usize = 0;
    for i in 0..3 {
        if (idx + i < content.len() && content[idx + i].is_ascii_digit()) {
            if content[idx + i] == ',' {
                break;
            }
            nbr = nbr * 10 + content[idx + i].to_digit(10).unwrap_or(0);
            len += 1;
        } else {
            break;
        }
    }
    return (nbr, len);
}

fn parse_mul(content: &Vec<char>, i: usize) -> u32 {
    if content[i] != 'm' || content[i + 1] != 'u' || content[i + 2] != 'l' || content[i + 3] != '('
    {
        return 0;
    }
    println!(
        "first 4 : {}{}{}{}",
        content[i],
        content[i + 1],
        content[i + 2],
        content[i + 3]
    );
    let mut idx = i + 4;
    let (a, l) = parse_int(content, idx);
    println!("Int parsed: {}", a);
    idx += l;
    if content[idx] != ',' {
        return 0;
    }
    idx += 1;
    let (b, j) = parse_int(content, idx);
    idx += j;
    if content[idx] != ')' {
        return 0;
    }
    idx += 1;
    for j in i..idx {
        print!("{}", content[j]);
    }
    println!(" = {} * {} = {}", a, b, a * b);
    return a * b;
}

fn parse_do(content: &Vec<char>, i: usize) -> bool {
    if content[i] == 'd' && content[i + 1] == 'o' && content[i + 2] == '(' && content[i + 3] == ')'
    {
        return true;
    }
    return false;
}

fn parse_dont(content: &Vec<char>, i: usize) -> bool {
    if content[i] == 'd'
        && content[i + 1] == 'o'
        && content[i + 2] == 'n'
        && content[i + 3] == '\''
        && content[i + 4] == 't'
        && content[i + 5] == '('
        && content[i + 6] == ')'
    {
        return true;
    }
    return false;
}

fn main() {
    let filename = "src/in";
    let content: Vec<char> = fs::read_to_string(filename)
        .expect("Should have been able to read the file")
        .chars()
        .collect();
    println!("Hello, world!");

    let mut ans: u64 = 0;
    let mut enabeled = true;
    for i in 0..content.len() {
        if parse_do(&content, i) {
            enabeled = true;
        }
        if parse_dont(&content, i) {
            enabeled = false;
        }
        if enabeled {
            ans += parse_mul(&content, i) as u64;
        }
    }

    println!("Ans: {} .", ans);
}
