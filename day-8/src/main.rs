use std::{collections::HashMap, fs};

fn read_input() -> Vec<Vec<char>> {
    let filename = "src/in";
    let content = fs::read_to_string(filename).expect("Should have been able to read the file");
    let map: Vec<Vec<char>> = content
        .split("\n")
        .map(|line| line.trim().chars().collect())
        .collect();
    map
}

fn get_antennas(map: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let c = map[i][j];
            if c != '.' {
                antennas
                    .entry(c)
                    .and_modify(|v| v.push((i, j)))
                    .or_insert(vec![(i, j)]);
            }
        }
    }
    antennas
}

fn main() {
    println!("Day 8");
    let map = read_input();
    let n = map.len() as isize;
    let m = map[0].len() as isize;
    let antennas = get_antennas(&map);
    let mut grid = vec![vec!['.'; map[0].len()]; map.len()];

    for antenna in antennas {
        let positions = antenna.1;
        for i in 0..positions.len() {
            let a = (positions[i].0 as isize, positions[i].1 as isize);
            for j in 0..positions.len() {
                if i == j {
                    continue;
                }
                let b = (positions[j].0 as isize, positions[j].1 as isize);
                let d = ((a.0 - b.0), (a.1 - b.1));
                let mut r = 0;
                while a.0 + d.0 * r >= 0
                    && a.0 + d.0 * r < n
                    && a.1 + d.1 * r >= 0
                    && a.1 + d.1 * r < m
                {
                    grid[(a.0 + d.0 * r) as usize][(a.1 + d.1 * r) as usize] = '#';
                    r += 1;
                }
            }
        }
    }
    let mut ans = 0;
    for i in grid {
        for j in i {
            print!("{}", j);
            if j == '#' {
                ans += 1;
            }
        }
        println!("");
    }

    println!("Ans: {}.", ans);
}
