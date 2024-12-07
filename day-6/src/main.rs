use core::time;
use std::{fs, thread::sleep};

fn read_input() -> Vec<Vec<char>> {
    let filename = "src/in";
    let content = fs::read_to_string(filename).expect("Should have been able to read the file");
    let lines: Vec<Vec<char>> = content
        .split("\n")
        .map(|line| line.trim().chars().collect())
        .collect();
    lines
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

fn find_guard(map: &Vec<Vec<char>>) -> (isize, isize) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '^' {
                return (i as isize, j as isize);
            }
        }
    }
    (0, 0)
}

enum Direction {
    UP,
    LEFT,
    RIGHT,
    DOWN,
}

impl Direction {
    fn velocity(&self) -> (isize, isize) {
        match *self {
            Direction::UP => (-1, 0),
            Direction::DOWN => (1, 0),
            Direction::LEFT => (0, -1),
            Direction::RIGHT => (0, 1),
        }
    }
    fn next(&self) -> Direction {
        match *self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }
}

fn simulate(mut map: Vec<Vec<char>>, mut x: isize, mut y: isize) -> i32 {
    let mut direction = Direction::UP;
    let mut ans = 1;
    let ten_millis = time::Duration::from_millis(100);
    while true {
        std::process::Command::new("clear").status().unwrap();
        print_map(&map);
        sleep(ten_millis);
        let (i, j) = direction.velocity();
        if x as isize + i < 0
            || y + j < 0
            || x + i >= (map.len() as isize)
            || y + j >= (map[x as usize].len() as isize)
        {
            break;
        }
        if map[(x as isize + i) as usize][(y as isize + j) as usize] == '#' {
            direction = direction.next();
        } else {
            if map[(x as isize + i) as usize][(y as isize + j) as usize] == '.' {
                ans += 1;
            }
            map[(x as isize + i) as usize][(y as isize + j) as usize] = 'X';
            x += i;
            y += j;
        }
    }
    ans
}

fn simulate_loop(mut map: Vec<Vec<char>>, mut x: isize, mut y: isize) -> bool {
    let mut direction = Direction::UP;
    let mut itr = 0;
    while true {
        if itr > 130 * 130 * 3 {
            return true;
        }
        let (i, j) = direction.velocity();
        if x as isize + i < 0
            || y + j < 0
            || x + i >= (map.len() as isize)
            || y + j >= (map[x as usize].len() as isize)
        {
            break;
        }
        if map[(x as isize + i) as usize][(y as isize + j) as usize] == '#' {
            direction = direction.next();
        } else {
            // if map[(x as isize + i) as usize][(y as isize + j) as usize] == '.' {
            //     ans += 1;
            // }
            map[(x as isize + i) as usize][(y as isize + j) as usize] = 'X';
            x += i;
            y += j;
        }
        itr += 1;
    }
    false
}

fn main() {
    let map = read_input();
    let (x, y) = find_guard(&map);
    let ans = simulate(map.clone(), x, y);
    // let mut ans = 0;
    // for i in 0..map.len() {
    //     for j in 0..map[i].len() {
    //         if map[i][j] == '.' {
    //             map[i][j] = '#';
    //             if simulate_loop(map.clone(), x.clone(), y.clone()) {
    //                 ans += 1;
    //             }
    //             map[i][j] = '.';
    //         }
    //     }
    // }
    // print_map(&map);
    print!("Ans: {}", ans);
}
